mod sign;

#[cfg(test)]
mod test;

pub use sign::Signer;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    SignerError(String),
    EnvVarError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "ParseError: {}", e),
            Error::SignerError(e) => write!(f, "SignerError: {}", e),
            Error::EnvVarError(e) => write!(f, "EnvVarError: {}", e),
        }
    }
}
