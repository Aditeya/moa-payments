mod error;
pub mod models;
mod schema;
mod users;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub use error::Error;

use anyhow::Context;
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
        if let Err(e) = Self::run_migrations(db_url) {
            eprintln!("Failed to run DB migrations: {e:#?}");
        };
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = Pool::builder(config)
            .build()
            .context("Failed to created MoaDB Diesel Async Pool")?;

        Ok(Self { pool })
    }

    async fn get_conn(&self) -> Result<Object<AsyncPgConnection>, PoolError> {
        self.pool.get().await
    }

    fn run_migrations(db_url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>  {
        let mut conn = PgConnection::establish(db_url)?;
        pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
        conn.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }
}
