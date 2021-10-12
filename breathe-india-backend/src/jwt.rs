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
    JwtError(jsonwebtoken::errors::Error),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("JWT_SECRET env var not found")]
    SecretKeyEnvNotFound,
    #[error("Token expired")]
    TokenExpired,
    #[error("Error decoding using jsonwebtoken: {0}")]
    JwtError(jsonwebtoken::errors::Error),
}

impl Claims {
    pub fn encode(&self) -> Result<String, EncodeError> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| EncodeError::SecretKeyEnvNotFound)?;
        let encoding_key = EncodingKey::from_secret(secret.as_ref());

        encode(&Header::default(), self, &encoding_key).map_err(EncodeError::JwtError)
    }

    pub fn decode(token: &str) -> Result<Self, DecodeError> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| DecodeError::SecretKeyEnvNotFound)?;
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        decode(token, &decoding_key, &Validation::default())
            .map(|t| t.claims)
            .map_err(|e| {
                if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = e.kind() {
                    DecodeError::TokenExpired
                } else {
                    DecodeError::JwtError(e)
                }
            })
    }
}
