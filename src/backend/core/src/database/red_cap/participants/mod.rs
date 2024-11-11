use crate::red_cap_data::{
    DegreeLevel, Ethnicity, Gender, HealthInsurance, PreferredLanguage, Programs, Race, Status,
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
mod extra;
pub mod goals;
pub mod health_overview;
pub mod medications;
pub mod overview;
use sqlx::prelude::FromRow;
/// Database Table: `participants`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Participants {
    pub id: i64,
    /// The ID within Red Cap. This is separate so if we added creating a new participant
    /// We know what users have been added to redcap or not
    pub red_cap_id: i64,
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
    pub signed_up_on: chrono::NaiveDateTime,
    /// For Database Only
    pub added_to_db_at: DateTime<FixedOffset>,
    /// For Database Only
    pub last_synced_with_redcap: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ParticipantDemograhics {
    pub id: i64,
    /// 1:1 with [Participants]
    pub participant_id: i64,
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
