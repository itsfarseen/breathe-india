use super::fetch_keys::JwkKey;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Claims {
    // The audience the token was issued for
    pub aud: String,
    // The expiry date -- as epoch seconds
    pub exp: i64,
    // The token issuer
    pub iss: String,
    // The subject the token refers to
    pub sub: String,
    // Issued at -- as epoch seconds
    pub iat: i64,
}

pub enum VerificationError {
    InvalidToken(String),
    KeyNotFound,
    InvalidSignature(jsonwebtoken::errors::Error),
    UnknownKeyAlgorithm,
}

pub struct JwtVerifier {
    pub audience: String,
    pub issuer: String,
}

impl JwtVerifier {
    pub fn verify_jwt(
        &self,
        token: &str,
        keys: &HashMap<String, JwkKey>,
    ) -> Result<TokenData<Claims>, VerificationError> {
        let token_kid = decode_header(token)
            .map(|header| header.kid)
            .map_err(|_| VerificationError::InvalidToken("Failed to decode header".to_owned()))?
            .ok_or_else(|| VerificationError::InvalidToken("header.kid not present".to_owned()))?;

        let key = keys.get(&token_kid).ok_or(VerificationError::KeyNotFound)?;

        let algorithm =
            Algorithm::from_str(&key.alg).map_err(|_| VerificationError::UnknownKeyAlgorithm)?;

        let mut validation = Validation::new(algorithm);
        validation.set_audience(&[self.audience]);
        validation.iss = Some(self.issuer.clone());
        let key = DecodingKey::from_rsa_components(&key.n, &key.e);

        decode::<Claims>(token, &key, &validation).map_err(VerificationError::InvalidSignature)
    }
}
