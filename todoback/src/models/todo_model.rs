
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime,Utc};

#[derive(Serialize, Deserialize, Debug, FromRow, Default)]
pub struct ToDo {
    pub id: i32,
    pub uuid: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Default)]
pub struct ToDoCreateBody {
    pub title: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, Debug, FromRow, Default)]
pub struct ToDoUpdateBody {
    pub title: String,
    pub description: String,
}