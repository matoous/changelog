use crate::models::Entry;
use derive_more::{Display, From};
use ormlite::model::*;
use ormlite::{postgres::Postgres, Pool};
use uuid::Uuid;

#[derive(Display, From, Debug)]
pub enum Error {
    Pg(ormlite::SqlxError),
    Ormlite(ormlite::Error),
}

impl std::error::Error for Error {}

#[derive(Clone)]
pub struct Repository {
    db_pool: Pool<Postgres>,
}

impl Repository {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }

    pub async fn add_entry(&self, entry: Entry) -> Result<Entry, Error> {
        let mut conn = self.db_pool.acquire().await?;
        let entry = Entry::builder()
            .id(Uuid::now_v7())
            .text(entry.text)
            .tags(entry.tags)
            .insert(&mut conn)
            .await?;
        Ok(entry)
    }

    pub async fn get_changelog(&self) -> Result<Vec<Entry>, Error> {
        let mut conn = self.db_pool.acquire().await?;
        let people = Entry::select()
            .order_desc("created_at")
            .fetch_all(&mut conn)
            .await?;
        Ok(people)
    }
}

#[cfg(test)]
mod test {}
