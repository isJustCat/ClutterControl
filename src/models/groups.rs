use uuid::Uuid;

pub struct Group {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<String>,
}