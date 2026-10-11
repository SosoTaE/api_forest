use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


#[derive(Serialize)]
pub struct Spec {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Deserialize, Validate)]
pub struct CreateSpec {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateSpec {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub description: Option<String>,
}


#[derive(Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
}