#![allow(unused)]
#[macro_use] extern crate rocket;

use std::default;

use anyhow::Result;
use cluttercontrol_base::{App, Config};
use endpoints::{r_create, r_delete, r_get, r_update};
use rocket::{http::{ContentType, Status}, response::{self, Builder, Responder}, Request, Response, tokio::sync::{RwLockReadGuard, RwLockWriteGuard}};
use serde_json::Value;
mod endpoints;

#[derive(Debug)]
pub struct ApiResponse {
    json: Value,
    status: Status
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let json_response = self.json.respond_to(req)?;

        let response = Builder::new(json_response)
            .status(self.status)
            .header(ContentType::JSON)
            .finalize(); 

        Ok(response)
    }
}
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
        .manage(App::launch().await)
        .mount("/", routes![r_create, r_get, r_update, r_delete])
        .launch()
        .await?;
    Ok(())
}