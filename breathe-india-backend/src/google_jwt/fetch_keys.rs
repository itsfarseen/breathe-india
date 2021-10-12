use anyhow::{Context, Result};
use reqwest::header::HeaderValue;
use reqwest::Response;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct JwkKey {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

pub struct JwkKeys {
    pub keys: HashMap<String, JwkKey>,
    pub created: Instant,
    pub validity: Duration,
}

impl JwkKeys {
    pub fn is_still_valid(&self, now: Instant) -> bool {
        self.created + self.validity > now
    }

    pub async fn fetch(jwk_url: &str, now: Instant) -> Result<JwkKeys> {
        let http_response = reqwest::get(jwk_url)
            .await
            .with_context(|| format!("Downloading {}", jwk_url))?;
        let max_age = get_max_age(&http_response).unwrap_or(FALLBACK_TIMEOUT);
        let result = http_response
            .json::<KeyResponse>()
            .await
            .context("Loading json body")?;

        let keys = result
            .keys
            .into_iter()
            .map(|key| (key.kid.clone(), key))
            .collect::<HashMap<_, _>>();

        Ok(JwkKeys {
            keys,
            created: now,
            validity: max_age,
        })
    }
}

const FALLBACK_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug, PartialEq)]
enum MaxAgeParseError {
    NoMaxAgeSpecified,
    NoCacheControlHeader,
    MaxAgeValueEmpty,
    NonNumericMaxAge,
}

// Determines the max age of an HTTP response
fn get_max_age(response: &Response) -> Result<Duration, MaxAgeParseError> {
    let headers = response.headers();
    let header = headers.get("Cache-Control");

    match header {
        Some(header_value) => parse_cache_control_header(header_value),
        None => Err(MaxAgeParseError::NoCacheControlHeader),
    }
}

fn parse_max_age_value(cache_control_value: &str) -> Result<Duration, MaxAgeParseError> {
    let tokens: Vec<&str> = cache_control_value.split(",").collect();
    for token in tokens {
        let key_value: Vec<&str> = token.split("=").map(|s| s.trim()).collect();
        let key = key_value.first().unwrap();

        if key.to_lowercase() != "max-age" {
            continue;
        }

        let val = key_value.get(1);
        match val {
            Some(value) => {
                let age = value
                    .parse()
                    .map_err(|_| MaxAgeParseError::NonNumericMaxAge)?;
                return Ok(Duration::from_secs(age));
            }
            None => return Err(MaxAgeParseError::MaxAgeValueEmpty),
        }
    }
    return Err(MaxAgeParseError::NoMaxAgeSpecified);
}

fn parse_cache_control_header(header_value: &HeaderValue) -> Result<Duration, MaxAgeParseError> {
    match header_value.to_str() {
        Ok(string_value) => parse_max_age_value(string_value),
        Err(_) => Err(MaxAgeParseError::NoCacheControlHeader),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_max_age_value() -> Result<(), MaxAgeParseError> {
        let res_exp = &[
            ("max-age=120", 120),
            (" max-age = 130 ", 130),
            ("key1=val,max-age=140,key2=val", 140),
            ("key1=val, max-age=150 , key2=val", 150),
        ];
        for (res, expected) in res_exp {
            assert_eq!(parse_max_age_value(res)?, Duration::from_secs(*expected));
        }
        Ok(())
    }

    #[test]
    fn test_parse_max_age_value_fail() {
        let res_exp = &[
            ("max-age", MaxAgeParseError::MaxAgeValueEmpty),
            ("max-age=1r", MaxAgeParseError::NonNumericMaxAge),
            ("max-age=", MaxAgeParseError::NonNumericMaxAge),
            ("key=val", MaxAgeParseError::NoMaxAgeSpecified),
        ];
        for (res, expected) in res_exp {
            assert_eq!(parse_max_age_value(res).unwrap_err(), *expected);
        }
    }
}
