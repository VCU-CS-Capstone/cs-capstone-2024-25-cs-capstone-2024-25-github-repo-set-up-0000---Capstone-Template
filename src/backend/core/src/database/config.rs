use serde::{Deserialize, Serialize};
use sqlx::postgres::PgConnectOptions;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum DBConfigError {
    #[error("Invalid host must be in the format host:port got `{0}`")]
    InvalidHost(String),
}
/// The configuration for the database.
///
/// Currently only supports PostgreSQL.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub database: String,
    // The host can be in the format host:port or just host.
    pub host: String,
    // The port is optional. If not specified the default port is used. or will be extracted from the host.
    pub port: Option<u16>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            user: "postgres".to_string(),
            password: "password".to_string(),
            database: "cs_25_303".to_string(),
            host: "localhost:5432".to_string(),
            port: None,
        }
    }
}
impl TryFrom<DatabaseConfig> for PgConnectOptions {
    type Error = DBConfigError;
    fn try_from(settings: DatabaseConfig) -> Result<PgConnectOptions, Self::Error> {
        // The port can be specified in the host field. If it is, we need to extract it.
        let host = settings.host.split(':').collect::<Vec<&str>>();

        let (host, port) = match host.len() {
            // The port is not specified. Use the default port.
            1 => (host[0], settings.port.unwrap_or(5432)),
            // The port is specified within the host. The port option is ignored.
            2 => (host[0], host[1].parse::<u16>().unwrap_or(5432)),
            _ => {
                // Not in the format host:port. Possibly IPv6 but we don't support that. As not really supported in the wild.
                return Err(DBConfigError::InvalidHost(settings.host));
            }
        };
        let options = PgConnectOptions::new()
            .username(&settings.user)
            .password(&settings.password)
            .host(host)
            .port(port)
            .database(&settings.database);

        Ok(options)
    }
}
