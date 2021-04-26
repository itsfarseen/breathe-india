#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    rocket::build()
        .mount("/hello", routes![world])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
async fn world() -> String {
    "Hello World".into()
}
