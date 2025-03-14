use std::time::Duration;

use super::*;

#[tokio::test]
async fn test() -> Result<(), Error> {

    let host = std::env::var("DB_HOST").unwrap();
    let user = std::env::var("DB_USER").unwrap();
    let region = std::env::var("DB_REGION").unwrap();

    let signer = Signer::builder()
        .expires_in(Duration::from_secs(900))
        .host(host)
        .port(5432u16)
        .user(user)
        .region(region)
        .build();
    let token = signer.fetch_token().await?;
    println!("{}", token);
    assert!(token.len() > 0);
    Ok(())
}
