#![feature(error_iter)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use dotenv::dotenv;
use google_jwt::JwkKeys;
use once_cell::sync::OnceCell;
use rocket::State;
use rocket_contrib::json::Json;
use slog::o;
use slog::Drain;
use slog::Logger;
use sqlx::postgres::PgPoolOptions;
use std::sync::Mutex;
use std::time::Instant;
use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;

mod google_jwt;
mod myres;
mod slog_nested;
use google_jwt::JwtVerifier;
use myres::{myerr, myok, MyRes};

struct GoogleJwkKeys(RwLock<Arc<JwkKeys>>);

const GOOGLE_JWK_URL: &'static str = "https://www.googleapis.com/oauth2/v3/certs";

impl GoogleJwkKeys {
    pub async fn load_new() -> Result<Self, Box<dyn Error>> {
        let keys = google_jwt::JwkKeys::fetch(GOOGLE_JWK_URL, Instant::now()).await?;
        Ok(Self(RwLock::new(Arc::new(keys))))
    }

    pub async fn get_latest_keys<'a>(&'a self) -> Result<Arc<JwkKeys>, Box<dyn Error + 'a>> {
        let read_lock = self.0.read().await;
        if read_lock.is_still_valid(Instant::now()) {
            return Ok((*read_lock).clone());
        }
        drop(read_lock);
        let mut write_lock = self.0.write().await;
        let keys = google_jwt::JwkKeys::fetch(GOOGLE_JWK_URL, Instant::now()).await?;
        *write_lock = Arc::new(keys);
        drop(write_lock);
        let read_lock = self.0.read().await;
        return Ok((*read_lock).clone());
    }
}

static LOGGER: OnceCell<Logger> = OnceCell::new();

static GOOGLE_JWK_KEYS: OnceCell<GoogleJwkKeys> = OnceCell::new();
static JWT_VERIFIER: OnceCell<JwtVerifier> = OnceCell::new();

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let error_log_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("error_log.json")
        .expect("Open log file");

    let error_log = slog_json::Json::new(error_log_file).build();
    let error_log = Mutex::new(error_log).map(slog::Fuse);
    let error_log = Logger::root(error_log, o!());
    let _ = LOGGER.set(error_log);

    let google_jwk_keys = GoogleJwkKeys::load_new().await?;
    let _ = GOOGLE_JWK_KEYS.set(google_jwk_keys);

    let jwt_verifier = JwtVerifier {
        audience: std::env::var("GOOGLE_JWT_AUDIENCE")?,
        issuer: std::env::var("GOOGLE_JWT_ISSUER")?,
    };
    let _ = JWT_VERIFIER.set(jwt_verifier);

    rocket::build()
        .mount("/hello", routes![world])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}

#[derive(Deserialize)]
struct Login {
    token: String,
}

#[post("/login", data = "<data>")]
async fn login(data: Json<Login>) -> MyRes<String, String> {
    let keys = GOOGLE_JWK_KEYS.get().unwrap();
    let keys = bail!(keys.get_latest_keys().await);
    let jwt_verifier = JWT_VERIFIER.get().unwrap();
    jwt_verifier.verify_jwt(&data.token, &keys.keys);
    todo!()
}

#[get("/")]
async fn world() -> String {
    "Hello World".into()
}
