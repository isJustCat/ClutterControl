#![allow(unused)]

use std::{collections::HashMap, env, fs::{File, OpenOptions}, hash::Hash, io::Read, path::Path, str::FromStr, sync::{RwLockReadGuard, RwLockWriteGuard}};
use models::{Action, Change, Entity, Loggable};
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

// We're back to a struct agaiin!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage(pub HashMap<Uuid, Entity>);


impl Storage {
    
    pub fn new() -> Self {
        return match Self::load() {
            Ok(s) => s,
            _ => Self(HashMap::new())
        }
    }

    pub fn load() -> Result<Storage> {
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("storage.json").expect("Failed to access storage.json in current directory!");
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let storage = serde_json::from_str(&data)?;
        Ok(storage)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("storage.json").expect("Failed to access storage.json in current directory!");
        serde_json::to_writer_pretty(file, &self.0)?;
        Ok(())
    }
}


impl Default for Config {
    fn default() -> Self {
        Self::new()
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
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("config.json").expect("Failed to access config.json in current directory!");
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let config = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("config.json").expect("Failed to access config.json in current directory!");
        serde_json::to_writer_pretty(file, self)?;
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

    pub async fn log_change<T: Loggable> (&self, entity_id: &str, action_str: &str, datetime: &str) {
        let action = Action::from_str(action_str).expect("Failed to parse action");
        let e_uuid = Uuid::from_str(entity_id).expect("Failed to parse Entity UUID");

        let mut storage = &mut self.storage.write().await.0;

        if let Some(entity) = storage.get_mut(&e_uuid) {
            let change = Change {
                entity_id: entity_id.to_string(),
                action,
                date: datetime.to_string(),
            };

            entity.add_change(change);
        } else {
            eprintln!("Entity with UUID {} not found", entity_id);
        }
    }
   
}
