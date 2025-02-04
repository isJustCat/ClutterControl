#![allow(unused)]

use std::{env, fs::{File, OpenOptions}, io::Read, path::Path, sync::RwLock};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use storage::Storage;

pub mod storage;
pub mod models;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub environment: String,
    pub app_name: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            environment: env::var("CC_ENV").as_deref().unwrap_or("prod").to_owned(),
            app_name: env::var("CC_APPNAME").as_deref().unwrap_or("ClutterControl").to_owned(),
        }
    }

    pub fn load() -> Result<Self> {
        let mut file = File::open(Path::new("config.json"))?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let config = serde_json::from_str(&data).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let file = OpenOptions::new().read(true).write(true).create(true).open("config.json")?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub storage: RwLock<Storage>,
    pub config: Config
}

impl App {
    pub fn launch() -> Self {
        let storage = Storage::load().unwrap_or(Storage::new());
        let config = Config::load().unwrap_or(Config::new());

        App {
            storage: storage.into(),
            config
        }
    }
}
