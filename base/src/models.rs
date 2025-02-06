
use std::{collections::HashMap, fmt::Display, str::FromStr};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]  /*
                            Internal tagging :3
                            The enum variant is inferred from a 'type' field.
                            Some funny serde "magic"..
                            https://serde.rs/enum-representations.html
                        */
pub enum Entity {
    Creature(Creature),
    Item(Item),
    Location(Location),
    Change(Change),
    Condition(Condition),
    Tag(Tag),
}
impl Entity {
    /*
    pub fn from_str(str: &str) -> Option<Self> {
        match str.to_lowercase().as_str() {
            "creature" => Some(Self::Creature(_)),
            "item" => Some(Self::Item),
            "location" => Some(Self::Location),
            "change" => Some(Self::Change),
            "condition" => Some(Self::Condition),
            "tag" => Some(Self::Tag),
            _ => None
        }
    } */ 
    pub fn as_str(&self) -> &str {
        match self {
            Entity::Creature(_) => "creture",
            Entity::Item(_) => "item",
            Entity::Location(_) => "location",
            Entity::Change(_) => "change",
            Entity::Condition(_) => "condition",
            Entity::Tag(_) => "tag"
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

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "new" => Ok(Condition::New),
            "excellent" => Ok(Condition::Excellent),
            "good" => Ok(Condition::Good),
            "fair" => Ok(Condition::Fair),
            "poor" => Ok(Condition::Poor),
            _ => Err(())
            
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Condition::New => "New",
            Condition::Excellent => "Excellent",
            Condition::Good => "Good",
            Condition::Fair => "Fair",
            Condition::Poor => "Poor",
        })
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

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "increment" => Ok(Action::Increment),
            "decrement" => Ok(Action::Decrement),
            "update" => Ok(Action::Update),
            "create" => Ok(Action::Create),
            "delete" => Ok(Action::Delete),
            _ => Err(())

        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Action::Increment => "Increment",
            Action::Decrement => "Decrement",
            Action::Update => "Update",
            Action::Delete => "Delete",
            Action::Create => "Create"
        })
    }
}


/// A Change that has occurred at some point in time
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Change {
    pub entity_id: String, // uuid identifying *what* was changed 
    pub action: Action, // what have you done?????
    pub date: String, // when have you done? :3

}
