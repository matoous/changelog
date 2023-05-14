use chrono::{DateTime, Utc};
use ormlite::model::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Model, Deserialize, Serialize, Debug, Eq, PartialEq, Default)]
#[ormlite(table = "entries")]
pub struct Entry {
    pub id: Uuid,
    pub tags: Vec<String>,
    pub text: String,
    #[ormlite(default)]
    pub created_at: DateTime<Utc>,
    #[ormlite(default)]
    pub updated_at: DateTime<Utc>,
}
