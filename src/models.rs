use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, Eq, PartialEq, Default)]
#[pg_mapper(table = "entries")]
pub struct Entry {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub title: String,
    pub description: Option<String>,
}

impl From<Row> for Entry {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            tags: row.get("tags"),
            title: row.get("title"),
            description: row.get("description"),
        }
    }
}
