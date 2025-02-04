
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Creature {
    pub id: usize,
    pub name: String,
    pub can_login: String,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub history: Vec<Change>,
}
 
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Change {
    pub id: usize,
    pub action: Action,
    pub date: String,

}

/// A physical location possibly containing items

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub items: Option<Vec<Item>>,
    pub properties: Option<Vec<Property>>,
    pub history: Vec<Change>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<usize>,
    pub properties: Option<Vec<usize>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property {
    pub id: usize,
    pub item_ids: Option<Vec<usize>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub history: Vec<Change>,
    pub fields: HashMap<String, String>
}