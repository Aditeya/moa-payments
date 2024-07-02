mod error;
pub mod models;
mod schema;
mod users;
pub use error::Error;

use anyhow::{Context, Ok};
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool, PoolError},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

pub struct MoaDB {
    pool: Pool<AsyncPgConnection>,
}

impl MoaDB {
    pub fn new(db_url: &str) -> anyhow::Result<Self> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = Pool::builder(config)
            .build()
            .context("Failed to created MoaDB Diesel Async Pool")?;
        Ok(Self { pool })
    }

    async fn get_conn(&self) -> Result<Object<AsyncPgConnection>, PoolError> {
        self.pool.get().await
    }
}
