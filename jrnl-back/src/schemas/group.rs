use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub owner_id: Uuid,
}

// pub struct GroupMembership {
//     pub group_id: Uuid,
//     pub user_id: Uuid,
// }
