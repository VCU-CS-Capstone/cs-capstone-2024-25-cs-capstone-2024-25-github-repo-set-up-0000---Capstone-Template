mod config;
pub mod red_cap;
pub mod user;
pub use config::*;

use sqlx::{migrate::Migrator, postgres::PgConnectOptions, PgPool};
use tracing::info;
pub static MIGRATOR: Migrator = sqlx::migrate!();
/// The type for a DateTime in the database.
///
/// Postgres Type: `TIMESTAMP WITH TIME ZONE`
pub type DBTime = chrono::DateTime<chrono::FixedOffset>;

pub type DBResult<T> = Result<T, sqlx::Error>;
pub async fn connect(
    config: PgConnectOptions,
    run_migrations: bool,
) -> Result<PgPool, sqlx::Error> {
    let database = PgPool::connect_with(config).await?;
    // TODO: Add Migration code here
    if run_migrations {
        info!("Running migrations");
        MIGRATOR.run(&database).await?;
    }
    Ok(database)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::database::{DatabaseConfig, MIGRATOR};

    #[tokio::test]
    #[ignore = "This test needs a database to run"]
    pub async fn run_migrations() -> anyhow::Result<()> {
        let test_env = crate::env_utils::read_env_file_in_core("test.env")?;

        let config: DatabaseConfig =
            serde_env::from_iter_with_prefix(test_env.iter(), "MIGRATIONS")?;
        let database = PgPool::connect_with(config.try_into()?).await?;

        MIGRATOR.run(&database).await?;

        Ok(())
    }
}
