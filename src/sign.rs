//! Implements the AWS RDS IAM authentication token generation.
//!
//! This module provides the core functionality for generating authentication tokens
//! that can be used to connect to AWS RDS instances using IAM authentication.

use std::time::Duration;
use std::time::SystemTime;

use aws_config::BehaviorVersion;
use aws_credential_types::provider::ProvideCredentials;
use aws_credential_types::Credentials;
use aws_sigv4::http_request::{sign, SignableBody, SignableRequest, SigningSettings};
use aws_sigv4::sign::v4;

/// A configured signer for generating RDS IAM authentication tokens.
///
/// The signer contains all the necessary configuration to generate authentication
/// tokens for connecting to an RDS instance. It can be created using the builder
/// pattern via [`SignerBuilder`].
#[derive(Debug)]
pub struct Signer {
    /// The duration for which the generated token will be valid.
    /// Defaults to 900 seconds (15 minutes).
    expires_in: Duration,
    /// The hostname of the RDS instance to connect to.
    /// This should be the endpoint provided by AWS.
    host: String,
    /// The port number the database is listening on.
    /// Common values are `5432` for `PostgreSQL` and `3306` for `MySQL`.
    port: u16,
    /// The database user to authenticate as.
    /// This user must be configured in RDS with IAM authentication enabled.
    user: String,
    /// The AWS region where the RDS instance is located.
    /// If not provided, will attempt to use the region from AWS configuration.
    region: Option<String>,
}

impl Default for Signer {
    fn default() -> Self {
        Self {
            expires_in: Duration::from_secs(900),
            host: "localhost".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            region: None,
        }
    }
}

/// Builder for creating a configured [`Signer`].
///
/// Provides a fluent interface for setting all necessary configuration
/// parameters for the signer.
#[derive(Debug)]
pub struct SignerBuilder {
    signer: Signer,
}

impl SignerBuilder {
    /// Creates a new `SignerBuilder` with default values.
    #[must_use]
    fn new() -> Self {
        Self {
            signer: Signer::default(),
        }
    }

    /// Sets the token expiration duration.
    ///
    /// # Arguments
    /// * `expires_in` - The duration for which the token will be valid
    #[must_use]
    pub fn expires_in(mut self, expires_in: impl Into<Duration>) -> Self {
        self.signer.expires_in = expires_in.into();
        self
    }

    /// Sets the RDS instance hostname.
    ///
    /// # Arguments
    /// * `host` - The RDS endpoint (e.g., "mydb.123456789012.us-east-1.rds.amazonaws.com")
    #[must_use]
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.signer.host = host.into();
        self
    }

    /// Sets the database port number.
    ///
    /// # Arguments
    /// * `port` - The port number (e.g., 5432 for `PostgreSQL`)
    #[must_use]
    pub fn port(mut self, port: impl Into<u16>) -> Self {
        self.signer.port = port.into();
        self
    }

    /// Sets the AWS region.
    ///
    /// # Arguments
    /// * `region` - The AWS region (e.g., "us-east-1")
    #[must_use]
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.signer.region = Some(region.into());
        self
    }

    /// Sets the database username.
    ///
    /// # Arguments
    /// * `user` - The database user to authenticate as
    #[must_use]
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.signer.user = user.into();
        self
    }

    /// Builds the final [`Signer`] instance.
    #[must_use]
    pub fn build(self) -> Signer {
        self.signer
    }
}

impl Signer {
    /// Creates a new `SignerBuilder` for configuring a Signer instance.
    #[must_use]
    pub fn builder() -> SignerBuilder {
        SignerBuilder::new()
    }

    /// Generates an authentication token for connecting to the RDS instance.
    ///
    /// This method will use the configured AWS credentials to generate a signed
    /// authentication token that can be used to connect to the RDS instance.
    /// The token is valid for the duration specified in the configuration.
    ///
    /// # Returns
    /// * `Ok(String)` - The authentication token
    /// * `Err(Error)` - If token generation fails
    ///
    /// # Errors
    /// * `SignerError` - If signing the request fails
    /// * `ParseError` - If URL parsing fails
    pub async fn fetch_token(&self) -> Result<String, super::Error> {
        let config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
        let credentials: Credentials = config
            .credentials_provider()
            .ok_or_else(|| super::Error::SignerError("no credentials provider found".to_string()))?
            .provide_credentials()
            .await
            .map_err(|e| super::Error::SignerError(e.to_string()))?;
        let identity = credentials.into();
        let region = self.region.clone().unwrap_or_else(|| {
            config
                .region()
                .map_or_else(|| "us-east-1".to_string(), ToString::to_string)
        });

        let mut signing_settings = SigningSettings::default();
        signing_settings.expires_in = Some(self.expires_in);
        signing_settings.signature_location =
            aws_sigv4::http_request::SignatureLocation::QueryParams;

        let signing_params = v4::SigningParams::builder()
            .identity(&identity)
            .region(&region)
            .name("rds-db")
            .time(SystemTime::now())
            .settings(signing_settings)
            .build()
            .map_err(|e| super::Error::SignerError(e.to_string()))?;

        let url = format!(
            "https://{hostname}:{port}/?Action=connect&DBUser={username}",
            hostname = self.host,
            port = self.port,
            username = self.user
        );

        let signable_request =
            SignableRequest::new("GET", &url, std::iter::empty(), SignableBody::Bytes(&[]))
                .map_err(|e| super::Error::SignerError(e.to_string()))?;

        let (signing_instructions, _signature) = sign(signable_request, &signing_params.into())
            .map_err(|e| super::Error::SignerError(e.to_string()))?
            .into_parts();

        let mut url = url::Url::parse(&url).map_err(|e| super::Error::ParseError(e.to_string()))?;
        for (name, value) in signing_instructions.params() {
            url.query_pairs_mut().append_pair(name, value);
        }

        let response = url.to_string().split_off("https://".len());

        Ok(response)
    }
}
