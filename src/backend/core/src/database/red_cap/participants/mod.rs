use crate::{
    database::prelude::*,
    red_cap_data::{
        DegreeLevel, Ethnicity, Gender, HealthInsurance, PreferredLanguage, Programs, Race, Status,
    },
};
use chrono::{DateTime, FixedOffset};
use cs25_303_macros::Columns;
use serde::{Deserialize, Serialize};
mod extra;
pub mod goals;
pub mod health_overview;
mod lookup;
pub mod medications;
mod new;
pub use lookup::*;
pub mod overview;
use crate::database::tools::ColumnType;
pub use new::*;
use sqlx::{postgres::PgRow, prelude::FromRow};
pub trait ParticipantType: for<'r> FromRow<'r, PgRow> + Unpin + Send + Sync {
    fn get_id(&self) -> i32;
    /// Leaving this to the default implementation is not recommended. As it will return all columns
    fn columns() -> Vec<ParticipantsColumn> {
        ParticipantsColumn::all()
    }

    async fn find_by_id(id: i32, database: &sqlx::PgPool) -> DBResult<Option<Self>> {
        let columns = concat_columns(&Self::columns(), None);
        let result = sqlx::query_as(&format!("SELECT {columns} FROM participants WHERE id = $1"))
            .bind(id)
            .fetch_optional(database)
            .await?;
        Ok(result)
    }
}
/// Database Table: `participants`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]
pub struct Participants {
    pub id: i32,
    /// The ID within Red Cap. This is separate so if we added creating a new participant
    /// We know what users have been added to redcap or not
    pub red_cap_id: Option<i32>,
    /// Redcap: first_name
    pub first_name: String,
    /// Red Cap last_name
    pub last_name: String,
    /// RedCap: phone1
    pub phone_number_one: Option<String>,
    /// RedCap: phone2
    pub phone_number_two: Option<String>,
    /// RedCap: other_info
    pub other_contact: Option<String>,
    pub program: Programs,
    /// Redcap: rhwp_location
    /// Relates to [super::Locations]
    pub location: Option<i32>,
    /// Red Cap: pt_status
    pub status: Option<Status>,
    /// Red Cap: behav_health_risk
    pub behavioral_risks_identified: Option<String>,
    /// Red Cap: consent_cc
    pub date_care_coordination_consent_signed: Option<chrono::NaiveDate>,
    /// Red Cap: consent_home
    pub date_home_visit_consent_signed: Option<chrono::NaiveDate>,
    /// Red CAp: date_intake
    pub signed_up_on: chrono::NaiveDate,
    /// For Database Only
    pub added_to_db_at: DateTime<FixedOffset>,
    /// For Database Only
    pub last_synced_with_redcap: Option<DateTime<FixedOffset>>,
}
impl ParticipantType for Participants {
    fn get_id(&self) -> i32 {
        self.id
    }
}

pub trait ParticipantDemograhicsType: for<'r> FromRow<'r, PgRow> + Unpin + Send + Sync {
    fn get_id(&self) -> i32;
    fn columns() -> Vec<ParticipantDemograhicsColumn> {
        ParticipantDemograhicsColumn::all()
    }

    async fn find_by_participant(id: i32, database: &sqlx::PgPool) -> DBResult<Option<Self>> {
        let columns = concat_columns(&Self::columns(), None);
        let result = sqlx::query_as(&format!(
            "SELECT {columns} FROM participant_demograhics WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(database)
        .await?;
        Ok(result)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]
pub struct ParticipantDemograhics {
    pub id: i32,
    /// 1:1 with [Participants]
    pub participant_id: i32,
    /// Redcap: age
    pub age: Option<i16>,
    /// Redcap Gender
    pub gender: Option<Gender>,
    /// Redcap: Race
    pub race: Option<Race>,
    /// Not Sure???
    pub race_other: Option<String>,
    pub race_multiple: Option<String>,
    /// Red Cap: ethnicity
    pub ethnicity: Option<Ethnicity>,
    pub language: Option<PreferredLanguage>,
    /// Red Cap: veteran
    /// Yes Or No
    pub is_veteran: Option<bool>,
    /// Red Cap: insurance
    pub health_insurance: Vec<HealthInsurance>,
    /// Red Cap: education
    pub highest_education_level: Option<DegreeLevel>,
}

impl ParticipantDemograhicsType for ParticipantDemograhics {
    fn get_id(&self) -> i32 {
        self.id
    }
}
