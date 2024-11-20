use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

use crate::{
    database::prelude::*,
    red_cap::Programs,
    red_cap::{RedCapDataSet, RedCapType},
};

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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]
pub struct Locations {
    pub id: i32,
    pub name: String,
    pub program: Programs,
    pub parent_location: Option<i32>,
    pub red_cap_connection_rules: Json<RedCapLocationConnectionRules>,
}
impl TableType for Locations {
    type Columns = LocationsColumn;
    fn table_name() -> &'static str {
        "locations"
    }
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
        SimpleSelectQueryBuilder::new(Locations::table_name(), &LocationsColumn::all())
            .where_equals(LocationsColumn::ParentLocation, parent_id)
            .query_as()
            .fetch_all(database)
            .await
    }
    pub async fn get_all(database: &sqlx::PgPool) -> Result<Vec<Locations>, sqlx::Error> {
        SimpleSelectQueryBuilder::new(Locations::table_name(), &LocationsColumn::all())
            .query_as()
            .fetch_all(database)
            .await
    }
    pub async fn find_all_in_program(
        program: Programs,
        database: &sqlx::PgPool,
    ) -> Result<Vec<Locations>, sqlx::Error> {
        SimpleSelectQueryBuilder::new(Locations::table_name(), &LocationsColumn::all())
            .where_equals(LocationsColumn::Program, program)
            .query_as()
            .fetch_all(database)
            .await
    }
}
/// So In Red Cap locations are split over multiple questions.
///
/// This is my easy way to convert the Red Cap locations into a single location.
///
/// This will also leave the door open for more locations to be added in the future.
///
/// Each field corresponds to the field name in Red Cap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RedCapLocationConnectionRules {
    pub rhwp_location: Option<usize>,
    pub rhwp_location_visit: Option<usize>,
    pub mhwp_location: Option<usize>,
    pub mhwp_location_visit: Option<usize>,
    pub mhwp_location_petersburg: Option<usize>,
    pub mhwp_location_visit_petersburg: Option<usize>,
}
impl RedCapLocationConnectionRules {
    pub fn does_match_no_visit(&self, location: &RedCapLocationConnectionRules) -> bool {
        self.rhwp_location == location.rhwp_location
            && self.mhwp_location == location.mhwp_location
            && self.mhwp_location_petersburg == location.mhwp_location_petersburg
    }
}

impl RedCapType for RedCapLocationConnectionRules {
    fn read(data: &impl RedCapDataSet) -> Option<Self>
    where
        Self: Sized,
    {
        let value = Self {
            rhwp_location: data.get_number("rhwp_location"),
            rhwp_location_visit: data.get_number("rhwp_location_visit"),
            mhwp_location: data.get_number("mhwp_location"),
            mhwp_location_visit: data.get_number("mhwp_location_visit"),
            mhwp_location_petersburg: data.get_number("mhwp_location_petersburg"),
            mhwp_location_visit_petersburg: data.get_number("mhwp_location_visit_petersburg"),
        };

        Some(value)
    }
}
