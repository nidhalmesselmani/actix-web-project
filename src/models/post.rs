use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PostModel {
    pub id: Uuid,
    pub message: String,
    pub username: String,
    pub day: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostSchema {
    pub message: String,
    pub username: String,
    pub day: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePostSchema {
    pub message: Option<String>,
    pub username: Option<String>,
    pub day: Option<String>
}