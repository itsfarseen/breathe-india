use std::env;

#[derive(Debug)]
pub struct JwkConfiguration {
    pub jwk_url: String,
    pub audience: String,
    pub issuer: String,
}

impl Default for JwkConfiguration {
    fn default() -> Self {
        JwkConfiguration {
            jwk_url: expect_env_var("JWK_URL", "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com"),
            audience: expect_env_var("JWK_AUDIENCE", "my-firebase-app-12345"),
            issuer: expect_env_var("JWK_ISSUER", "https://securetoken.google.com/my-firebase-app-12345"),
        }
    }
}

#[cfg(debug_assertions)]
fn expect_env_var(name: &str, default: &str) -> String {
    return env::var(name).unwrap_or(String::from(default));
}

#[cfg(not(debug_assertions))]
fn expect_env_var(name: &str, _default: &str) -> String {
    return env::var(name).expect(&format!(
        "Environment variable {name} is not defined",
        name = name
    ));
}
