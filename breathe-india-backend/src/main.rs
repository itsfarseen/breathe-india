use sqlx::postgres::PgPoolOptions;
use std::io::Result;

async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env!("DATABASE_URL"))
        .await?;

    println!("Hello, world!");
    Ok(())
}
