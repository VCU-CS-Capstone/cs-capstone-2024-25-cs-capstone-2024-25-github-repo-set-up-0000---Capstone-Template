mod config;
pub mod red_cap;
pub mod user;
pub use config::*;

use sqlx::{postgres::PgConnectOptions, PgPool};
use tracing::info;
//static MIGRATOR: Migrator = sqlx::migrate!();
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
        //     MIGRATOR.run(pool).await?;
    }
    Ok(database)
}
