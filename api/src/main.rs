#![allow(unused)]
#[macro_use] extern crate rocket;

use std::default;

use anyhow::Result;
use cluttercontrol_base::{App, Config};
use endpoints::{r_create_object, r_delete_object, r_get_everything, r_query_storage};
mod endpoints;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .configure(rocket::Config {
            address: std::env::var("CC_BIND_ADDR")
                .map(|e| e.parse().unwrap())
                .unwrap_or("127.0.0.1".parse().unwrap()),
            port: std::env::var("CC_PORT")
                .map(|e| e.parse().unwrap())
                .unwrap_or(8123),
            ..Default::default()
        })
        .manage(App::launch())
        .mount("/", routes![r_create_object, r_query_storage, r_get_everything, r_delete_object])
        .launch()
        .await?;
    Ok(())
}