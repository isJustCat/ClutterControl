

#![allow(unused)]

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    pub id: u32,
    pub name: String,
    pub can_login: String,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub history: Vec<Change>,
}
 
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    New,
    Excellent,
    Good,
    Fair,
    Poor,
}

impl Condition {
    pub fn as_str(&self) -> &str {
        match self {
            Condition::New => "New",
            Condition::Excellent => "Excellent",
            Condition::Good => "Good",
            Condition::Fair => "Fair",
            Condition::Poor => "Poor",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Increment,
    Decrement,
    Update,
    Delete,
}

impl Action {
    pub fn as_str(&self) -> &str {
        match self {
            Action::Increment => "Increment",
            Action::Decrement => "Decrement",
            Action::Update => "Update",
            Action::Delete => "Delete",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Change {
    pub id: u32,
    pub action: Action,
    pub date: String,

}

/// A physical location possibly containing items

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub items: Option<Vec<Item>>,
    pub properties: Option<Vec<Property>>,
    pub history: Vec<Change>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<u32>,
    pub properties: Option<Vec<u32>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub id: u32,
    pub item_ids: Option<Vec<u32>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub history: Vec<Change>,
    pub fields: HashMap<String, String>
}