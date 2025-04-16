use std::env;
use sqlx::postgres::PgPool;
#[macro_use] extern crate rocket;

mod models;

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .launch()
        .await?;
    Ok(())
}
