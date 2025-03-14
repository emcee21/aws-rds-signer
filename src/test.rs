use std::time::Duration;

use super::*;

#[tokio::test]
async fn test() -> Result<(), Error> {
    let mut signer = Signer::default();
    if let Some(v) = std::env::var("EXPIRES_IN").ok().and_then(|v| {
        v.parse()
            .map(Duration::from_secs)
            .map_err(|_| Error::ParseError("EXPIRES_IN is not a valid number".to_string()))
            .ok()
    }) {
        signer.expires_in(v);
    }
    if let Ok(v) = std::env::var("HOST") {
        signer.host(v);
    }
    if let Ok(v) = std::env::var("PORT")
        .map_err(|_| Error::EnvVarError("PORT is not set".to_string()))
        .and_then(|v| {
            v.parse()
                .map_err(|_| Error::ParseError("PORT is not a valid number".to_string()))
        })
    {
        signer.port(v);
    }
    if let Ok(v) = std::env::var("USER") {
        signer.user(v);
    }
    if let Ok(v) = std::env::var("REGION") {
        signer.region(Some(v));
    }

    let token = signer.fetch_token().await?;
    println!("{}", token);
    assert!(token.len() > 0);
    Ok(())
}
