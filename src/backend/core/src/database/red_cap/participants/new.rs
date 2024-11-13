use chrono::{DateTime, FixedOffset, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::Executor;

use crate::{
    database::DBResult,
    red_cap_data::{
        DegreeLevel, Ethnicity, Gender, HealthInsurance, MedicationFrequency, MobilityDevice,
        PreferredLanguage, Programs, Race, Status,
    },
};

use super::Participants;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewParticipant {
    pub red_cap_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub phone_number_one: Option<String>,
    pub phone_number_two: Option<String>,
    pub other_contact: Option<String>,
    pub program: Programs,
    pub location: Option<i32>,
    pub status: Option<Status>,
    pub behavioral_risks_identified: Option<String>,
    pub date_care_coordination_consent_signed: Option<chrono::NaiveDate>,
    pub date_home_visit_consent_signed: Option<chrono::NaiveDate>,
    pub signed_up_on: chrono::NaiveDate,
    pub last_synced_with_redcap: Option<DateTime<FixedOffset>>,
}

impl NewParticipant {
    pub async fn insert_return_participant(
        self,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> DBResult<Participants> {
        let Self {
            red_cap_id,
            first_name,
            last_name,
            phone_number_one,
            phone_number_two,
            other_contact,
            program,
            location,
            status,
            behavioral_risks_identified,
            date_care_coordination_consent_signed,
            date_home_visit_consent_signed,
            signed_up_on,
            last_synced_with_redcap,
        } = self;

        let result = sqlx::query_as(
            "
                INSERT INTO participants (
                    red_cap_id,
                    first_name,
                    last_name,
                    phone_number_one,
                    phone_number_two,
                    other_contact,
                    program,
                    location,
                    status,
                    behavioral_risks_identified,
                    date_care_coordination_consent_signed,
                    date_home_visit_consent_signed,
                    signed_up_on,
                    last_synced_with_redcap
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                 RETURNING *;
            ",
        )
        .bind(red_cap_id)
        .bind(first_name)
        .bind(last_name)
        .bind(phone_number_one)
        .bind(phone_number_two)
        .bind(other_contact)
        .bind(program)
        .bind(location)
        .bind(status)
        .bind(behavioral_risks_identified)
        .bind(date_care_coordination_consent_signed)
        .bind(date_home_visit_consent_signed)
        .bind(signed_up_on)
        .bind(last_synced_with_redcap)
        .fetch_one(database)
        .await?;

        Ok(result)
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct NewDemographics {
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
impl NewDemographics {
    pub async fn insert_none(
        self,
        participant_id: i32,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> DBResult<()> {
        let Self {
            age,
            gender,
            race,
            race_other,
            race_multiple,
            ethnicity,
            language,
            is_veteran,
            health_insurance,
            highest_education_level,
        } = self;

        sqlx::query(
            "
                INSERT INTO participant_demographics (
                    participant_id,
                    age,
                    gender,
                    race,
                    race_other,
                    race_multiple,
                    ethnicity,
                    language,
                    is_veteran,
                    health_insurance,
                    highest_education_level
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    ",
        )
        .bind(participant_id)
        .bind(age)
        .bind(gender)
        .bind(race)
        .bind(race_other)
        .bind(race_multiple)
        .bind(ethnicity)
        .bind(language)
        .bind(is_veteran)
        .bind(health_insurance)
        .bind(highest_education_level)
        .execute(database)
        .await?;

        Ok(())
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct NewHealthOverview {
    pub height: Option<i32>,
    /// Red Cap: health_conditions
    pub reported_health_conditions: Option<String>,
    /// Red Cap: allergies
    pub allergies: Option<String>,
    /// Red Cap: info_mobility
    pub mobility_devices: Option<Vec<MobilityDevice>>,
    /// Red Cap: personal_cuff
    pub has_blood_pressure_cuff: Option<bool>,
    /// Red Cap: num_meds
    pub takes_more_than_5_medications: Option<bool>,
}
impl NewHealthOverview {
    pub async fn insert_none(
        self,
        participant_id: i32,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> DBResult<()> {
        let Self {
            height,
            reported_health_conditions,
            allergies,
            mobility_devices,
            has_blood_pressure_cuff,
            takes_more_than_5_medications,
        } = self;

        sqlx::query(
            "
                INSERT INTO participant_health_overview (
                    participant_id,
                    height,
                    reported_health_conditions,
                    allergies,
                    mobility_devices,
                    has_blood_pressure_cuff,
                    takes_more_than_5_medications
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                    ",
        )
        .bind(participant_id)
        .bind(height)
        .bind(reported_health_conditions)
        .bind(allergies)
        .bind(mobility_devices)
        .bind(has_blood_pressure_cuff)
        .bind(takes_more_than_5_medications)
        .execute(database)
        .await?;

        Ok(())
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewMedication {
    pub name: String,
    pub dosage: String,
    pub frequency: MedicationFrequency,
    pub date_prescribed: Option<chrono::NaiveDate>,
    pub date_entered_into_system: Option<NaiveDate>,
    pub is_current: Option<bool>,
    pub date_discontinued: Option<chrono::NaiveDate>,
    pub comments: Option<String>,
}
impl NewMedication {
    pub async fn insert_none(
        self,
        participant_id: i32,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> DBResult<()> {
        let Self {
            name,
            dosage,
            frequency,
            date_prescribed,
            date_entered_into_system,
            is_current,
            date_discontinued,
            comments,
        } = self;
        let date_entered_into_system =
            date_entered_into_system.unwrap_or_else(|| Local::now().date_naive());
        sqlx::query(
            "
                INSERT INTO participant_medications (
                    participant_id,
                    name,
                    dosage,
                    frequency,
                    date_prescribed,
                    date_entered_into_system,
                    is_current,
                    date_discontinued,
                    comments
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    ",
        )
        .bind(participant_id)
        .bind(name)
        .bind(dosage)
        .bind(frequency)
        .bind(date_prescribed)
        .bind(date_entered_into_system)
        .bind(is_current)
        .bind(date_discontinued)
        .bind(comments)
        .execute(database)
        .await?;

        Ok(())
    }
}
