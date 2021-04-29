#![feature(error_iter)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

use anyhow::{anyhow, Context, Result};
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use rocket::request::Outcome;
use rocket::Request;
use rocket::State;
use rocket::{http::Status, request::FromRequest};
use rocket_contrib::json::Json;
use slog::o;
use slog::Drain;
use slog::Logger;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use tokio::sync::RwLock;
use uuid::Uuid;

mod google_jwt;
mod jwt;
mod models;
mod myres;
mod slog_nested;
use google_jwt::Claims;
use google_jwt::JwkKeys;
use google_jwt::JwtVerifier;
use models::*;
use myres::HasStatusCode;
use myres::MyRes;

struct GoogleJwkKeys(RwLock<Arc<JwkKeys>>);

const GOOGLE_JWK_URL: &'static str = "https://www.googleapis.com/oauth2/v3/certs";

impl GoogleJwkKeys {
    pub async fn load_new() -> Result<Self> {
        let keys = google_jwt::JwkKeys::fetch(GOOGLE_JWK_URL, Instant::now()).await?;
        Ok(Self(RwLock::new(Arc::new(keys))))
    }

    pub async fn get_latest_keys<'a>(&self) -> Result<Arc<JwkKeys>> {
        let read_lock = self.0.read().await;
        if read_lock.is_still_valid(Instant::now()) {
            return Ok((*read_lock).clone());
        }
        drop(read_lock);
        let mut write_lock = self.0.write().await;
        let keys = google_jwt::JwkKeys::fetch(GOOGLE_JWK_URL, Instant::now())
            .await
            .context("Refreshing Google's JWKs")?;
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
async fn main() -> Result<()> {
    dotenv()?;
    let db_url = std::env::var("DATABASE_URL").context("Get DATABASE_URL env var")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .with_context(|| format!("Connect to postgres - {}", db_url))?;

    let error_log_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("error_log.json")
        .context("Open error_log.json")?;

    let error_log = slog_json::Json::new(error_log_file).build();
    let error_log = Mutex::new(error_log).map(slog::Fuse);
    let error_log = Logger::root(error_log, o!());
    let _ = LOGGER.set(error_log);

    let google_jwk_keys = GoogleJwkKeys::load_new()
        .await
        .context("Load Google's JWK for the first time")?;
    let _ = GOOGLE_JWK_KEYS.set(google_jwk_keys);

    let jwt_verifier = JwtVerifier {
        audience: std::env::var("GOOGLE_JWT_AUDIENCE")
            .context("Get GOOGLE_JWT_AUDIENCE env var")?,
        issuer: std::env::var("GOOGLE_JWT_ISSUER").context("Get GOOGLE_JWT_ISSUER env var")?,
    };
    let _ = JWT_VERIFIER.set(jwt_verifier);

    rocket::build()
        .mount("/hello", routes![login])
        .manage(pool)
        .launch()
        .await
        .context("Launch rocket")?;
    Ok(())
}

#[derive(Deserialize)]
struct Login {
    token: String,
}

#[derive(Serialize)]
struct LoginSuccess {
    our_token: String,
}

#[derive(Serialize)]
enum LoginErr {
    InvalidToken,
}

impl HasStatusCode for LoginErr {
    fn get_status(&self) -> Status {
        match self {
            LoginErr::InvalidToken => Status::Unauthorized,
        }
    }
}

impl HasStatusCode for () {
    fn get_status(&self) -> Status {
        panic!("No status code for ()")
    }
}

#[post("/login", data = "<data>")]
async fn login(data: Json<Login>, db: State<'_, PgPool>) -> MyRes<LoginSuccess, LoginErr> {
    let keys = GOOGLE_JWK_KEYS.get().unwrap();
    let keys = fail!(keys.get_latest_keys().await);
    let jwt_verifier = JWT_VERIFIER.get().unwrap();
    let token_data = bail!(jwt_verifier.verify_jwt(&data.token, &keys.keys), |_| {
        LoginErr::InvalidToken
    });
    let claims: Claims = token_data.claims;

    let userid: Option<Uuid> = fail!(
        sqlx::query!("SELECT id FROM users WHERE email = $1", &claims.email)
            .fetch_optional(&*db)
            .await
    )
    .map(|u| u.id);

    let userid = match userid {
        Some(userid) => userid,
        None => {
            fail!(
                sqlx::query!(
                    r#"
                    INSERT INTO users(name, email, profile_pic_url, bio)
                    VALUES($1, $2, $3, $4)
                    RETURNING id"#,
                    &claims.name,
                    &claims.email,
                    &claims.picture,
                    ""
                )
                .fetch_one(&*db)
                .await
            )
            .id
        }
    };

    let our_claims = jwt::Claims {
        sub: userid,
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let our_jwt = fail!(our_claims.encode());

    let resp = LoginSuccess { our_token: our_jwt };

    MyRes::Ok(resp)
}

struct LoggedInUser(Uuid);

#[async_trait]
impl<'r> FromRequest<'r> for LoggedInUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = match request.headers().get_one("Authorization") {
            Some(x) => x,
            None => {
                return Outcome::Failure((Status::BadRequest, ()));
            }
        };
        let token = if auth_header.starts_with("Bearer ") {
            &auth_header["Bearer ".len()..]
        } else {
            return Outcome::Failure((Status::BadRequest, ()));
        };
        let claims = match jwt::Claims::decode(token) {
            Ok(claims) => claims,
            Err(_) => {
                return Outcome::Failure((Status::BadRequest, ()));
            }
        };
        let userid = claims.sub;
        Outcome::Success(LoggedInUser(userid))
    }
}

#[get("/profile")]
async fn profile(user: LoggedInUser, db: State<'_, PgPool>) -> MyRes<User, ()> {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", &user.0)
        .fetch_optional(&*db)
        .await;
    let user = fail!(res);
    let user = fail!(user.ok_or_else(|| anyhow!("Logged in user not found in db")));
    MyRes::Ok(user)
}
