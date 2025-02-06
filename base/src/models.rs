
use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Entity {
    Creature,
    Item,
    Location,
    Change,
    Condition,
    Tag
}
impl Entity {
    pub fn from_str(str: &str) -> Option<Self> {
        match str.to_lowercase().as_str() {
            "creature" => Some(Self::Creature),
            "item" => Some(Self::Item),
            "location" => Some(Self::Location),
            "change" => Some(Self::Change),
            "condition" => Some(Self::Condition),
            "tag" => Some(Self::Tag),
            _ => None
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Entity::Creature => "creture",
            Entity::Item => "item",
            Entity::Location => "location",
            Entity::Change => "change",
            Entity::Condition => "condition",
            Entity::Tag => "tag"
        }
    }
}

/// Creatures have some sort of relation to items and some may log in to the app and also interact with those items in the digital realm
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Creature {
    pub name: String,
    pub can_login: String,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub history: Vec<Change>,
}

/// A physical location possibly containing items
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub name: String,
    pub description: Option<String>,
    pub items: Option<Vec<Item>>,
    pub history: Vec<Change>,

}

/// It's all about items...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<String>,
}


/// The condition an item is in
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

    pub fn from_str(str: &str) -> Option<Self> {
        match str.to_lowercase().as_str() {
            "new" => Some(Condition::New),
            "excellent" => Some(Condition::Excellent),
            "good" => Some(Condition::Good),
            "fair" => Some(Condition::Fair),
            "poor" => Some(Condition::Poor),
            _ => None
            
        }
    } 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub value: String
}

/// Action done by a creature
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Increment,
    Decrement,
    Create,
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
            Action::Create => "Create"
        }
    }

    pub fn from_str(str: &str) -> Option<Self> {
        match str.to_lowercase().as_str() {
            "increment" => Some(Action::Increment),
            "decrement" => Some(Action::Decrement),
            "update" => Some(Action::Update),
            "create" => Some(Action::Create),
            "delete" => Some(Action::Delete),
            _ => None

        }
    }
}

/// A Change that has occurred at some point in time
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Change {
    pub entity_id: String, // uuid identifying *what* was changed 
    pub action: Action, // what have you done?????
    pub date: String, // when have you done? :3

}
