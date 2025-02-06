#![allow(unused)]

use std::{collections::HashMap, env, fs::{File, OpenOptions}, io::Read, path::Path};
use models::{Action, Change, Entity};
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod models;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub environment: String,
    pub app_name: String,
}

pub type Storage = HashMap<Uuid, Entity>;

pub trait FsExt {
    fn save(&self) -> Result<()>;
    fn load() -> Result<Storage>;
}

impl FsExt for Storage {
    fn load() -> Result<Storage> {
        let mut file = File::open(Path::new("storage.json"))?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let storage = serde_json::from_str(&data)?;
        Ok(storage)
    }

    fn save(&self) -> Result<()> {
        let file = OpenOptions::new().write(true).create(true).open("storage.json")?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            environment: env::var("CC_ENV").unwrap_or_else(|_| "prod".to_owned()),
            app_name: env::var("CC_APPNAME").unwrap_or_else(|_| "ClutterControl".to_owned()),
        }
    }

    pub fn load() -> Result<Self> {
        let mut file = File::open(Path::new("config.json"))?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let config = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let file = OpenOptions::new().write(true).create(true).open("config.json")?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct App {
    pub storage: RwLock<Storage>,
    pub config: RwLock<Config>,
}

impl App {
    pub async fn launch() -> Self {
        let storage = Storage::load().unwrap_or_else(|_| Storage::new());
        let config = Config::load().unwrap_or_else(|_| Config::new());

        App {
            storage: RwLock::new(storage),
            config: RwLock::new(config),
        }
    }

   
}
