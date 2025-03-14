#![doc(html_root_url = "https://docs.rs/aws-rds-signer")]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]
#![allow(clippy::multiple_crate_versions)]

//! # AWS RDS Signer
//!
//! A Rust library for generating AWS IAM authentication tokens for Amazon RDS database connections.
//! This crate provides a secure and efficient way to generate authentication tokens for RDS IAM
//! database authentication.
//!
//! ## Features
//!
//! - Generate IAM authentication tokens for RDS database connections
//! - Support for AWS credentials from environment, instance profiles, and explicit configuration
//! - Thread-safe and async-ready implementation
//! - Zero unsafe code
//!
//! ## Example
//!
//! ```rust,no_run
//! use aws_rds_signer::{Signer, SignerBuilder};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let signer = Signer::builder()
//!         .region("us-east-1")
//!         .host("my-db.xxxxx.region.rds.amazonaws.com")
//!         .port(5432u16)
//!         .user("my_user")
//!         .build();
//!
//!     let token = signer.fetch_token().await?;
//!     println!("Authentication token: {}", token);
//!     Ok(())
//! }
//! ```

mod sign;

#[cfg(test)]
mod test;

pub use sign::{Signer, SignerBuilder};

/// Represents errors that can occur during the RDS signing process.
#[derive(Debug)]
pub enum Error {
    /// Error that occurs during parsing of input parameters or URLs.
    ParseError(String),
    /// Error that occurs during the signing process.
    SignerError(String),
    /// Error that occurs when retrieving environment variables.
    EnvVarError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(e) => write!(f, "ParseError: {e}"),
            Self::SignerError(e) => write!(f, "SignerError: {e}"),
            Self::EnvVarError(e) => write!(f, "EnvVarError: {e}"),
        }
    }
}

impl std::error::Error for Error {}
