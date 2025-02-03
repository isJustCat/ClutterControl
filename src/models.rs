pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

pub struct Category {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

pub struct Tag {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>
}

pub struct Status {
    pub id: u32,
    pub name: String,
    pub available: bool,
    pub description: Option<String>,
}

// a creature can own or (temporarily) posess an item
pub struct Creature {
    pub id: u32,
    pub name: String,
    pub can_login: bool,
    pub password_hash: Option<String>,
    pub email: Option<String>
}

pub struct CustomField {
    pub id: u32,
    pub name: String,
    pub value: String,
}

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

pub struct Repair {
    pub id: u32,
    pub date: String,
    pub summary: Option<String>,
}

pub struct RepairHistory {
    pub item_id: u32,
    pub repair_ids: Vec<u32>,
}

pub struct ItemCategory {
    pub item_id: u32,
    pub category_id: u32,
}

pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: Option<String>
}

pub struct ItemQuantity {
    pub item_id: u32,
    pub actual: u32,
    pub desired: Option<u32>
}

pub struct ItemValue {
    pub item_id: u32,
    pub purchase_date: Option<String>,
    pub price: Option<u32>,
    pub condition: Option<Condition>,
}

pub struct ItemCustomField {
    pub item_id: u32,
    pub field_id: u32,
}

pub struct ItemLocation {
    pub item_id: u32,
    pub location_id: u32,
}

pub struct ItemLabel {
    pub item_id: u32,
    pub label_id: u32,
}

pub struct ItemStatus {
    pub item_id: u32,
    pub status_id: u32,
}

pub struct ItemExpiry {
    pub item_id: u32,
    pub expiry_date: u32
}

pub struct ItemWarranty {
    pub active: Boolean,
    pub until_date: Option<String>
}

pub struct ItemOwner {
    pub item_id: u32,
    pub creature_id: u32
}

pub struct ItemCarrier {
    pub item_id: u32,
    pub creature_id: u32,
}