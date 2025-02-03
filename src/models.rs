use serde::{Serialize, Deserialize};
use std::any::Any

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub id: u32,
    pub name: String,
    pub available: bool,
    pub description: Option<String>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    pub id: u32,
    pub name: String,
    pub can_login: bool,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomField {
    pub id: u32,
    pub name: String,
    pub value: String,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemRepair {
    pub id: u32,
    pub item_id: u32
    pub date: String,
    pub summary: Option<String>,
    pub history: Vec<Change>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemCategory {
    pub id: u32,
    pub item_id: u32,
    pub category_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemQuantity {
    pub id: u32,
    pub item_id: u32,
    pub actual: u32,
    pub desired: Option<u32>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemValue {
    pub item_id: u32,
    pub purchase_date: Option<String>,
    pub price: Option<u32>,
    pub condition: Option<Condition>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemCustomField {
    pub item_id: u32,
    pub field_id: u32,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemLocation {
    pub item_id: u32,
    pub location_id: u32,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemLabel {
    pub item_id: u32,
    pub label_id: u32,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemStatus {
    pub item_id: u32,
    pub status_id: u32,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemExpiry {
    pub item_id: u32,
    pub expiry_date: u32,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemWarranty {
    pub active: bool,
    pub until_date: Option<String>,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemOwner {
    pub item_id: u32,
    pub creature_id: u32,
    pub history: Vec<Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemCarrier {
    pub item_id: u32,
    pub creature_id: u32,
    pub history: Vec<Change>,
}
