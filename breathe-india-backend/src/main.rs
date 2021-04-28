#![feature(error_iter)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use dotenv::dotenv;
use google_jwt::JwkKeys;
use once_cell::sync::OnceCell;
use rocket::response::Responder;
use rocket::Request;
use rocket::State;
use rocket_contrib::json::Json;
use slog::o;
use slog::Drain;
use slog::Logger;
use sqlx::postgres::PgPoolOptions;
use std::sync::RwLockReadGuard;
use std::sync::{Mutex, RwLock};
use std::time::Instant;
use std::{error::Error, sync::Arc};

mod google_jwt;
mod slog_nested;
use slog_nested::WrapSerde;

struct GoogleJwkKeys(Arc<RwLock<JwkKeys>>);

const GOOGLE_JWK_URL: &'static str = "https://www.googleapis.com/oauth2/v3/certs";

impl GoogleJwkKeys {
    pub async fn load_new() -> Result<Self, Box<dyn Error>> {
        let keys = google_jwt::JwkKeys::fetch(GOOGLE_JWK_URL, Instant::now()).await?;
        Ok(Self(Arc::new(RwLock::new(keys))))
    }

    pub async fn get_latest_keys<'a>(
        &'a self,
    ) -> Result<RwLockReadGuard<'_, JwkKeys>, Box<dyn Error + 'a>> {
        let read_lock = self.0.read()?;
        if read_lock.is_still_valid(Instant::now()) {
            return Ok(read_lock);
        }
        drop(read_lock);
        let mut write_lock = self.0.write()?;
        let keys = google_jwt::JwkKeys::fetch(GOOGLE_JWK_URL, Instant::now()).await?;
        *write_lock = keys;
        drop(write_lock);
        let read_lock = self.0.read()?;
        return Ok(read_lock);
    }
}

static LOGGER: OnceCell<Logger> = OnceCell::new();

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
async fn login(data: Json<Login>, google_keys: State<'_, GoogleJwkKeys>) -> MyRes<String> {
    todo!()
}

struct MyRes<T>(Result<T, Box<dyn Error>>);

impl<'r, T: Responder<'r, 'static>> Responder<'r, 'static> for MyRes<T> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self.0 {
            Ok(r) => r.respond_to(req),
            Err(e) => {
                let logger = LOGGER.get().unwrap();
                // todo!();
                let route_name = req
                    .route()
                    .and_then(|r| r.name.as_ref().map(|r| r.to_string()))
                    .unwrap_or_default()
                    .to_string();
                let uri = req.uri().to_string();
                let method = req.method().to_string();

                let error_chain: Vec<_> = e.chain().map(|e| e.to_string()).collect();

                slog::error!(logger, "E500"; "method" => method, "route_name" => route_name, "uri" => uri, "error_chain" => WrapSerde(error_chain));
                Err(rocket::http::Status::InternalServerError)
            }
        }
    }
}

#[get("/")]
async fn world() -> String {
    "Hello World".into()
}
