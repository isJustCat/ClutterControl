#![allow(unused)]

use anyhow::Result;
use models::{Action, Change, Entity, Loggable};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::{File, OpenOptions},
    hash::Hash,
    io::Read,
    path::Path,
    str::FromStr,
    sync::{RwLockReadGuard, RwLockWriteGuard},
};
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod auth;
pub mod models;

// We're back to a struct agaiin!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage(pub HashMap<Uuid, Entity>);

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        match Self::load() {
            Ok(s) => s,
            _ => Self(HashMap::new()),
        }
    }

    pub fn load() -> Result<Storage> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open("storage.json")
            .expect("Failed to access storage.json in current directory!");
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let storage = serde_json::from_str(&data)?;
        Ok(storage)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open("storage.json")
            .expect("Failed to access storage.json in current directory!");
        serde_json::to_writer_pretty(file, &self.0)?;
        Ok(())
    }

    pub fn match_entity_type(&self, str: &str, mut f: impl FnMut(&Uuid, &Entity)) {
        for (uuid, stored_entity) in self.0.iter() {
            match stored_entity {
                Entity::Creature(_) if str.eq_ignore_ascii_case("creature") => {
                    (f(uuid, stored_entity))
                }
                Entity::Item(_) if str.eq_ignore_ascii_case("item") => f(uuid, stored_entity),
                Entity::Location(_) if str.eq_ignore_ascii_case("location") => {
                    f(uuid, stored_entity)
                }
                Entity::Change(_) if str.eq_ignore_ascii_case("change") => f(uuid, stored_entity),
                Entity::Condition(_) if str.eq_ignore_ascii_case("condition") => {
                    f(uuid, stored_entity)
                }
                Entity::Tag(_) if str.eq_ignore_ascii_case("tag") => f(uuid, stored_entity),
                _ => {}
            }
        }
    }
}

// TODO Implement this
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub environment: String,
    pub app_name: String,
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
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open("config.json")
            .expect("Failed to access config.json in current directory!");
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let config = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open("config.json")
            .expect("Failed to access config.json in current directory!");
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

    // TODO: Actually make the endpoints utilize this function
    pub async fn log_change<T: Loggable>(&self, entity_id: &str, action_str: &str, datetime: &str) {
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
