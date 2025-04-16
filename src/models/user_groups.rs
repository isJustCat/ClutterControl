use uuid::Uuid;

pub struct user_group {
    pub uuid: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
}