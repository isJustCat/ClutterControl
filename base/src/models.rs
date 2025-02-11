
use std::{collections::HashMap, fmt::Display, str::FromStr};
use logproclol::Loggable;

use serde::{de, Deserialize, Serialize};

pub trait Loggable {
    fn add_change(&mut self, change: Change);
}


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

impl FromStr for Entity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "creature" => Ok(Entity::Creature(Creature::default())),
            "item" => Ok(Entity::Item(Item::default())),
            "location" => Ok(Entity::Location(Location::default())),
            "change" => Ok(Entity::Change(Change::default())),
            "condition" => Ok(Entity::Condition(Condition::default())),
            "tag" => Ok(Entity::Tag(Tag::default())),
            _ => Err(())

        }
    }
}

impl Loggable for Entity {
    fn add_change(&mut self, change: Change) {
        match self {
            Entity::Creature(creature) => creature.add_change(change),
            Entity::Item(item) => item.add_change(change),
            Entity::Location((location)) => location.add_change(change),
            _ => {}
        }
    }
}

/// Creatures have some sort of relation to items and some may log in to the app and also interact with those items in the digital realm
#[derive(Serialize, Deserialize, Debug, Clone, Default, Loggable)]
pub struct Creature {
    pub name: String,
    pub can_login: String,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub history: Vec<Change>,
}

/// A physical location possibly containing items
#[derive(Serialize, Deserialize, Debug, Clone, Default, Loggable)]
pub struct Location {
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<Change>,
    pub items: Option<Vec<String>>
}


/// It's all about items...
#[derive(Serialize, Deserialize, Debug, Clone, Default, Loggable)]
pub struct Item {
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<Change>,
    pub location: String
}
/// The condition an item is in
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    #[default]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tag {
    pub name: String,
    pub value: String
}

/// Action done by a creature
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Increment,
    Decrement,
    #[default]
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Change {
    pub entity_id: String, // uuid identifying *what* was changed 
    pub action: Action, // what have you done?????
    pub date: String, // when have you done? :3
}
