use uuid::Uuid;

pub struct Owner {
    pub uuid: Uuid,
    pub kind: String,
    pub id: Uuid
}