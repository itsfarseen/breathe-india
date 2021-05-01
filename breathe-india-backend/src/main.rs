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
use std::sync::Mutex;
use std::time::Instant;
use std::{collections::HashMap, sync::Arc};
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

    let error_log = slog_json::Json::default(error_log_file);
    let error_log = Mutex::new(error_log).map(slog::Fuse);
    let error_log = Logger::root(error_log, o!());
    let _ = LOGGER.set(error_log);

    let google_jwk_keys = GoogleJwkKeys::load_new()
        .await
        .context("Load Google's JWK for the first time")?;
    let _ = GOOGLE_JWK_KEYS.set(google_jwk_keys);

    let audience =
        std::env::var("GOOGLE_JWT_AUDIENCE").context("Get GOOGLE_JWT_AUDIENCE env var")?;
    let issuers_str =
        std::env::var("GOOGLE_JWT_ISSUERS").context("Get GOOGLE_JWT_ISSUER env var")?;
    let issuers = issuers_str
        .split(' ')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let jwt_verifier = JwtVerifier { audience, issuers };
    let _ = JWT_VERIFIER.set(jwt_verifier);

    let allowed_origins_str =
        std::env::var("CORS_ALLOWED_ORIGINS").context("Get CORS_ALLOWED_ORIGINS env var")?;
    let allowed_origins = allowed_origins_str
        .split(' ')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let allowed_origins = rocket_cors::AllowedOrigins::some_exact(&allowed_origins);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Patch,
            rocket::http::Method::Delete,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        allow_credentials: false,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .mount(
            "/",
            routes![
                login,
                profile,
                profile_update,
                posts,
                my_posts,
                posts_create,
                posts_update,
                posts_delete
            ],
        )
        .manage(pool)
        .attach(cors)
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
            Err(jwt::DecodeError::TokenExpired) => {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
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

#[derive(Deserialize)]
pub struct UpdateProfile {
    pub bio: String,
}

#[post("/profile", data = "<data>")]
async fn profile_update(
    data: Json<UpdateProfile>,
    user: LoggedInUser,
    db: State<'_, PgPool>,
) -> MyRes<User, ()> {
    let res = sqlx::query_as!(
        User,
        "UPDATE users SET bio=$2 WHERE id = $1 RETURNING *",
        &user.0,
        &data.bio
    )
    .fetch_optional(&*db)
    .await;
    let user = fail!(res);
    let user = fail!(user.ok_or_else(|| anyhow!("Logged in user not found in db")));
    MyRes::Ok(user)
}

#[derive(Serialize, Deserialize, sqlx::Type, FromFormField)]
#[sqlx(rename_all = "lowercase")]
pub enum PostType {
    Needs,
    Supplies,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: Uuid,
    userid: Uuid,
    post_type: PostType,
    state: String,
    district: String,
    city: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    message: String,
}

#[derive(Serialize)]
pub struct ProfilePublic {
    id: Uuid,
    name: String,
    profile_pic_url: String,
    bio: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostFull {
    id: Uuid,
    userid: Uuid,
    post_type: PostType,
    state: String,
    district: String,
    city: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    message: String,
    items: Vec<PostItem>,
}

#[derive(Serialize, Deserialize)]
pub struct PostItem {
    id: Uuid,
    post_id: Uuid,
    item: String,
    quantity: String,
}

#[derive(Serialize)]
pub struct GetPosts {
    posts: Vec<PostFull>,
    users: HashMap<Uuid, ProfilePublic>,
}

async fn get_posts_full(posts: Vec<Post>, db: &PgPool) -> Result<Vec<PostFull>> {
    let post_ids: Vec<Uuid> = posts.iter().map(|p| p.id).collect();

    let res = sqlx::query_as!(
        PostItem,
        r#"
        SELECT * FROM post_items
        WHERE post_id = ANY($1)
        "#,
        &post_ids
    )
    .fetch_all(db)
    .await;
    let post_items = res?;

    let mut posts_full: HashMap<Uuid, PostFull> = HashMap::new();
    for post in posts {
        posts_full.insert(
            post.id,
            PostFull {
                id: post.id,
                userid: post.userid,
                post_type: post.post_type,
                state: post.state,
                district: post.district,
                city: post.city,
                created_at: post.created_at,
                updated_at: post.updated_at,
                message: post.message,
                items: Vec::new(),
            },
        );
    }

    if !posts_full.is_empty() {
        for post_item in post_items {
            posts_full
                .get_mut(&post_item.post_id)
                .map(|p| p.items.push(post_item));
        }
    }

    let posts_full = posts_full.into_iter().map(|(_k, v)| v).collect();

    Ok(posts_full)
}

#[get("/posts?<start>&<n>&<typ>")]
async fn posts(
    start: Option<i64>,
    n: Option<i64>,
    typ: PostType,
    db: State<'_, PgPool>,
) -> MyRes<GetPosts, ()> {
    let res = sqlx::query_as!(
        Post,
        r#"
        SELECT posts.id,
               userid,
               post_type as "post_type: _",
               state,
               district,
               city,
               created_at,
               updated_at,
               message
        FROM posts 
        WHERE post_type = $3
        OFFSET $1
        LIMIT $2
        "#,
        start,
        n,
        typ: _
    )
    .fetch_all(&*db)
    .await;
    let posts = fail!(res);

    let userids: Vec<Uuid> = posts.iter().map(|p| p.userid).collect();
    let res = sqlx::query_as!(
        ProfilePublic,
        r#"
        SELECT id, name, bio, profile_pic_url FROM users
        WHERE id = ANY($1)
        "#,
        &userids
    )
    .fetch_all(&*db)
    .await;
    let users = fail!(res);
    let users = users.into_iter().map(|u| (u.id, u)).collect();

    let posts_full = fail!(get_posts_full(posts, &*db).await);

    MyRes::Ok(GetPosts {
        users,
        posts: posts_full,
    })
}

#[get("/my_posts")]
async fn my_posts(user: LoggedInUser, db: State<'_, PgPool>) -> MyRes<Vec<PostFull>, ()> {
    let res = sqlx::query_as!(
        Post,
        r#"
        SELECT posts.id,
               userid,
               post_type as "post_type: _",
               state,
               district,
               city,
               created_at,
               updated_at,
               message
        FROM posts 
        WHERE userid = $1
        "#,
        user.0
    )
    .fetch_all(&*db)
    .await;
    let posts = fail!(res);

    let posts_full = fail!(get_posts_full(posts, &*db).await);

    MyRes::Ok(posts_full)
}

#[derive(Serialize, Deserialize)]
pub struct PostNew {
    post_type: PostType,
    state: String,
    district: String,
    city: String,
    message: String,
    items: Vec<PostItemNew>,
}

#[derive(Serialize, Deserialize)]
pub struct PostItemNew {
    item: String,
    quantity: String,
}

#[post("/posts", data = "<data>")]
async fn posts_create(
    user: LoggedInUser,
    db: State<'_, PgPool>,
    data: Json<PostNew>,
) -> MyRes<PostFull, ()> {
    let res = sqlx::query_as!(
        Post,
        r#"INSERT INTO posts(
            userid, 
            post_type,
            state, 
            district,
            city,
            message
        ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING 
               id,
               userid,
               post_type as "post_type: _",
               state,
               district,
               city,
               created_at,
               updated_at,
               message
        "#,
        user.0,
        data.post_type: _,
        data.state,
        data.district,
        data.city,
        data.message
    )
    .fetch_one(&*db)
    .await;

    let post = fail!(res);
    let mut items = Vec::new();
    for item in &data.items {
        let res = sqlx::query_as!(
            PostItem,
            r#"INSERT INTO post_items(
            post_id, 
            item,
            quantity
        ) VALUES ($1, $2, $3) RETURNING *"#,
            post.id,
            item.item,
            item.quantity
        )
        .fetch_one(&*db)
        .await;
        let item = fail!(res);
        items.push(item);
    }

    let post_full = PostFull {
        id: post.id,
        userid: post.userid,
        post_type: post.post_type,
        state: post.state,
        district: post.district,
        city: post.city,
        created_at: post.created_at,
        updated_at: post.updated_at,
        message: post.message,
        items,
    };
    MyRes::Ok(post_full)
}

#[derive(Serialize)]
enum PostUpdateError {
    NotFound,
}

impl HasStatusCode for PostUpdateError {
    fn get_status(&self) -> Status {
        match self {
            PostUpdateError::NotFound => Status::NotFound,
        }
    }
}

#[patch("/posts/<id>", data = "<data>")]
async fn posts_update(
    id: rocket_contrib::uuid::Uuid,
    user: LoggedInUser,
    db: State<'_, PgPool>,
    data: Json<PostNew>,
) -> MyRes<PostFull, PostUpdateError> {
    let id: Uuid = id.into_inner();
    let res = sqlx::query_as!(
        Post,
        r#"UPDATE posts SET 
            post_type = $3,
            state = $4,
            district = $5,
            city = $6,
            message = $7,
            updated_at = $8
         WHERE id = $1 AND userid = $2
         RETURNING 
               id,
               userid,
               post_type as "post_type: _",
               state,
               district,
               city,
               created_at,
               updated_at,
               message
        "#,
        id,
        user.0,
        data.post_type: _,
        data.state,
        data.district,
        data.city,
        data.message,
        chrono::Utc::now()
    )
    .fetch_optional(&*db)
    .await;

    let post = fail!(res);
    let post = bail!(post.ok_or(()), |_| PostUpdateError::NotFound);

    let res = sqlx::query!(r#"DELETE FROM post_items WHERE post_id = $1"#, id,)
        .execute(&*db)
        .await;
    let _ = fail!(res);

    let mut items = Vec::new();
    for item in &data.items {
        let res = sqlx::query_as!(
            PostItem,
            r#"INSERT INTO post_items(
                post_id, 
                item,
                quantity
            ) VALUES ($1, $2, $3) RETURNING *"#,
            post.id,
            item.item,
            item.quantity
        )
        .fetch_one(&*db)
        .await;
        let item = fail!(res);
        items.push(item);
    }

    let post_full = PostFull {
        id: post.id,
        userid: post.userid,
        post_type: post.post_type,
        state: post.state,
        district: post.district,
        city: post.city,
        created_at: post.created_at,
        updated_at: post.updated_at,
        message: post.message,
        items,
    };
    MyRes::Ok(post_full)
}

#[delete("/posts/<id>")]
async fn posts_delete(
    id: rocket_contrib::uuid::Uuid,
    user: LoggedInUser,
    db: State<'_, PgPool>,
) -> MyRes<(), PostUpdateError> {
    let id: Uuid = id.into_inner();
    let res = sqlx::query!(
        r#"DELETE FROM posts
        WHERE id = $1 AND userid = $2"#,
        id,
        user.0,
    )
    .execute(&*db)
    .await;

    let res = fail!(res);
    if res.rows_affected() == 0 {
        return MyRes::Err(PostUpdateError::NotFound);
    }
    MyRes::Ok(())
}
