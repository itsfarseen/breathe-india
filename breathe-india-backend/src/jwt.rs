use std::env::VarError;

use jsonwebtoken::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("JWT_SECRET env var not found")]
    SecretKeyEnvNotFound,
    #[error("Error encoding using jsonwebtoken: {0}")]
    JWTError(jsonwebtoken::errors::Error),
}

impl From<VarError> for EncodeError {
    fn from(_: VarError) -> Self {
        Self::SecretKeyEnvNotFound
    }
}

impl From<jsonwebtoken::errors::Error> for EncodeError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::JWTError(e)
    }
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("JWT_SECRET env var not found")]
    SecretKeyEnvNotFound,
    #[error("Token expired")]
    TokenExpired,
    #[error("Error decoding using jsonwebtoken: {0}")]
    JWTError(jsonwebtoken::errors::Error),
}

impl From<VarError> for DecodeError {
    fn from(_: VarError) -> Self {
        Self::SecretKeyEnvNotFound
    }
}

impl From<jsonwebtoken::errors::Error> for DecodeError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Self::TokenExpired,
            _ => Self::JWTError(e),
        }
    }
}

impl Claims {
    pub fn encode(&self) -> Result<String, EncodeError> {
        let secret = std::env::var("JWT_SECRET")?;
        let encoding_key = EncodingKey::from_secret(secret.as_ref());

        Ok(encode(&Header::default(), self, &encoding_key)?)
    }

    pub fn decode(token: &str) -> Result<Self, DecodeError> {
        let secret = std::env::var("JWT_SECRET")?;
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        Ok(decode(token, &decoding_key, &Validation::default())?.claims)
    }
}
