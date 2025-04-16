use uuid::Uuid;
use crate::models::owners::Owner;

pub struct Item {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    pub tags: String
}