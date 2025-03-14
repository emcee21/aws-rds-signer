# AWS RDS Signer

A Rust library for generating AWS IAM authentication tokens for Amazon RDS database connections. This library simplifies the process of authenticating to RDS instances using IAM roles and credentials.

## Features

- Generate IAM authentication tokens for RDS database connections
- Support for custom expiration times
- Automatic region detection from AWS configuration
- Simple builder-style API
- Async/await support
- Zero dependencies on AWS SDK (uses lightweight AWS signature v4 implementation)


## Relevant Links

* https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.IAMDBAuth.Connecting.html
* https://github.com/aws/aws-sdk-js-v3/blob/main/packages/rds-signer/src/Signer.ts
* https://github.com/awslabs/aws-sdk-rust/issues/951
* https://github.com/awslabs/aws-sdk-rust/issues/147
* https://github.com/deadpool-rs/deadpool/issues/396


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
aws-rds-signer = "0.1.0"
```

## Usage

Here's a basic example of how to use the library:

```rust
use aws_rds_signer::Signer;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), aws_rds_signer::Error> {
    let mut signer = Signer::default();
    
    // Configure the signer
    signer
        .host("your-db-instance.region.rds.amazonaws.com".to_string())
        .port(5432)
        .user("your_db_user".to_string())
        .expires_in(Duration::from_secs(900)) // 15 minutes
        .region(Some("us-east-1".to_string())); // Optional, will use default region if not specified

    // Fetch the authentication token
    let token = signer.fetch_token().await?;
    
    // Use the token in your database connection string
    println!("Authentication token: {}", token);
    Ok(())
}
```

## Configuration

The `Signer` struct supports the following configuration options:

- `host`: The hostname of your RDS instance
- `port`: The port number the database is listening on
- `user`: The database username
- `expires_in`: Token expiration duration (defaults to 900 seconds)
- `region`: AWS region (optional, will use the region from your AWS configuration)

## Requirements

- Rust 2021 edition or later
- AWS credentials configured in your environment (either through environment variables, AWS CLI configuration, or IAM role)
- Appropriate IAM permissions to connect to your RDS instance

## License

This project is licensed under the MIT License - see the LICENSE file for details.

