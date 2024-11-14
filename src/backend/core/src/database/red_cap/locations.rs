use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::red_cap_data::Programs;

/// Red Cap ID: pilot_gaps_coordination
/// # Values for RHWP
/// - Church Hill House
/// - Dominion Place
/// - Highland Park
/// - 4th Ave
/// - Health Hub
/// - The Rosa
/// # Values for MHWP
/// - Lawrenceville
/// - Petersburg
/// - Tappahannock
/// - Southwood
///
/// TODO: Petersburg has sub locations.
/// # Values for Petersburg
///- VCRC
///- Police substation
///- Gilhaven
///- VSU Van
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Locations {
    pub id: i32,
    pub name: String,
    pub program: Programs,
    pub parent_location: Option<i32>,
}
impl Locations {
    pub async fn find_by_name(
        name: &str,
        database: &sqlx::PgPool,
    ) -> Result<Option<Locations>, sqlx::Error> {
        let result = sqlx::query_as(
            r#"
            SELECT * FROM locations
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(database)
        .await?;
        Ok(result)
    }

    pub async fn find_children_of(
        parent_id: i32,
        database: &sqlx::PgPool,
    ) -> Result<Vec<Locations>, sqlx::Error> {
        let result = sqlx::query_as(
            r#"
            SELECT * FROM locations
            WHERE parent_location = $1
            "#,
        )
        .bind(parent_id)
        .fetch_all(database)
        .await?;
        Ok(result)
    }

    pub async fn find_all_in_program(
        program: Programs,
        database: &sqlx::PgPool,
    ) -> Result<Vec<Locations>, sqlx::Error> {
        let result = sqlx::query_as(
            r#"
            SELECT * FROM locations
            WHERE program = $1
            "#,
        )
        .bind(program)
        .fetch_all(database)
        .await?;
        Ok(result)
    }
}
