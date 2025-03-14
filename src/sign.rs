use std::time::Duration;
use std::time::SystemTime;

use aws_config::BehaviorVersion;
use aws_credential_types::provider::ProvideCredentials;
use aws_sigv4::http_request::{sign, SignableBody, SignableRequest, SigningSettings};
use aws_sigv4::sign::v4;

pub struct Signer {
    /**
     * Required. The duration of the token in seconds. Defaults to 900 seconds.
     */
    expires_in: Duration,
    /**
     * Required. The hostname of the database to connect to.
     */
    host: String,
    /**
     * Required. The port number the database is listening on.
     */
    port: u16,
    /**
     * Required. The username to login as.
     */
    user: String,
    /**
     * Optional. The region the database is located in. Uses the region inferred from the runtime if omitted.
     */
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

impl Signer {
    pub fn expires_in(&mut self, expires_in: Duration) -> &mut Self {
        self.expires_in = expires_in;
        self
    }
    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    pub fn region(&mut self, region: Option<String>) -> &mut Self {
        self.region = region;
        self
    }
    pub fn user(&mut self, user: String) -> &mut Self {
        self.user = user;
        self
    }
    pub async fn fetch_token(&self) -> Result<String, super::Error> {
        let config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
        let credentials = config
            .credentials_provider()
            .expect("no credentials provider found")
            .provide_credentials()
            .await
            .expect("unable to load credentials");
        let identity = credentials.into();
        let region = self.region.clone().unwrap_or(
            config
                .region()
                .map(|r| r.to_string())
                .unwrap_or("us-east-1".to_string()),
        );

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
                .expect("signable request");

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
