use uuid::Uuid;

pub struct Location {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Option<Uuid>,
    pub parent: Option<Uuid>,
    pub tags: Option<String>
}