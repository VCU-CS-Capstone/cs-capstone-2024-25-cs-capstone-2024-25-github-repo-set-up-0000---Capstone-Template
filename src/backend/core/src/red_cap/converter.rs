use sqlx::PgPool;
use thiserror::Error;

use crate::database::red_cap::locations::{Locations, RedCapLocationConnectionRules};
pub mod goals;
pub mod medications;
pub mod participants;
#[derive(Debug, Error)]
pub enum RedCapConverterError {
    #[error("Error in database: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Required field missing: {0}")]
    RequiredFieldMissing(&'static str),
}

pub struct RedCapConverter {
    pub database: PgPool,
    pub locations: Vec<Locations>,
}
impl RedCapConverter {
    pub async fn new(database: PgPool) -> Result<Self, RedCapConverterError> {
        let locations = Locations::get_all(&database).await?;
        let result = Self {
            database,
            locations,
        };

        Ok(result)
    }
    pub fn find_location_from_connection_rules(
        &self,
        location: &RedCapLocationConnectionRules,
    ) -> Option<Locations> {
        self.locations
            .iter()
            .find(|x| x.red_cap_connection_rules.does_match_no_visit(location))
            .cloned()
    }
}
