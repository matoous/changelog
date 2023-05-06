use crate::models::Entry;
use deadpool_postgres::PoolError;
use deadpool_postgres::{Client, Pool};
use derive_more::{Display, From};
use rusty_ulid::Ulid;
use tokio_pg_mapper::{Error as PgMapperError, FromTokioPostgresRow};
use tokio_postgres::Error as PgError;
use tracing::instrument;

#[derive(Display, From, Debug)]
pub enum Error {
    Pg(PgError),
    PgMapper(PgMapperError),
    Pool(PoolError),
}

impl std::error::Error for Error {}

#[derive(Clone)]
pub struct Repository {
    db_pool: Pool,
}

impl Repository {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }

    #[instrument(skip(self))]
    pub async fn get_changelog(&self) -> Result<Vec<Entry>, Error> {
        let client: Client = self.db_pool.get().await?;

        let entries = Self::list_entries_impl(&client).await?;

        Ok(entries)
    }

    #[instrument(skip(self, entry))]
    pub async fn add_entry(&self, mut entry: Entry) -> Result<Entry, Error> {
        entry.id = Ulid::generate().to_string();
        let client: Client = self.db_pool.get().await?;
        client
            .execute(
                "INSERT INTO entries (id, tags, title, description)",
                &[&entry.id, &entry.tags, &entry.title, &entry.description],
            )
            .await?;
        Ok(entry)
    }

    async fn list_entries_impl(client: &Client) -> Result<Vec<Entry>, Error> {
        client
            .query("SELECT * FROM entries", &[])
            .await?
            .iter()
            .map(|row| Ok(Entry::from_row_ref(row)?))
            .collect()
    }
}

#[cfg(test)]
mod test {}
