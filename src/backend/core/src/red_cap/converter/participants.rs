use chrono::{Local, NaiveDate};
use serde::Serialize;
use tracing::info;

use crate::{
    database::red_cap::locations::RedCapLocationConnectionRules,
    red_cap::{
        DegreeLevel, Ethnicity, HealthInsurance, MobilityDevice, PreferredLanguage, Programs,
        RedCapDataSet, RedCapGender, RedCapRace, RedCapType, Status,
    },
};

use super::{RedCapConverter, RedCapConverterError};

#[derive(Debug, Default, Serialize)]
pub struct RedCapParticipant {
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
    pub date_care_coordination_consent_signed: Option<NaiveDate>,
    /// Red Cap: consent_home
    pub date_home_visit_consent_signed: Option<NaiveDate>,
    /// Red Cap: date_intake
    pub signed_up_on: NaiveDate,
}

impl RedCapParticipant {
    pub async fn read_participant<D: RedCapDataSet>(
        data: &D,
        converter: &mut RedCapConverter,
    ) -> Result<Self, RedCapConverterError> {
        let first_name = data
            .get("first_name")
            .and_then(|x| x.to_string())
            .ok_or(RedCapConverterError::RequiredFieldMissing("first_name"))?;

        let last_name = data
            .get("last_name")
            .and_then(|x| x.to_string())
            .ok_or(RedCapConverterError::RequiredFieldMissing("last_name"))?;
        let program = data
            .get_enum("program")
            .ok_or(RedCapConverterError::RequiredFieldMissing("program"))?;
        let red_cap_id = data
            .get_number("record_id")
            .ok_or(RedCapConverterError::RequiredFieldMissing("record_id"))?;

        let location = if let Some(location) = RedCapLocationConnectionRules::read(data) {
            let location = converter.find_location_from_connection_rules(&location);
            info!("Location: {:?}", location);
            location.map(|x| x.id)
        } else {
            None
        };

        let result = Self {
            red_cap_id: Some(red_cap_id as i32),
            first_name,
            last_name,
            program,
            phone_number_one: data.get("phone1").and_then(|x| x.to_string()),
            phone_number_two: data.get("phone2").and_then(|x| x.to_string()),
            other_contact: data.get("other_info").and_then(|x| x.to_string()),
            location,
            status: data.get("pt_status").and_then(|x| x.to_enum::<Status>()),
            behavioral_risks_identified: data.get("behav_health_risk").and_then(|x| x.to_string()),
            date_care_coordination_consent_signed: data.get("consent_cc").and_then(|x| x.to_date()),
            date_home_visit_consent_signed: data.get("consent_home").and_then(|x| x.to_date()),
            signed_up_on: data
                .get("date_intake")
                .and_then(|x| x.to_date())
                .unwrap_or_else(|| Local::now().date_naive()),
        };

        Ok(result)
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RedCapParticipantDemographics {
    /// Redcap: age
    pub age: Option<i16>,
    /// Redcap Gender
    pub gender: Option<RedCapGender>,
    /// Redcap: Race
    pub race: Option<RedCapRace>,
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

impl RedCapParticipantDemographics {
    pub async fn read<D: RedCapDataSet>(data: &D) -> Result<Self, RedCapConverterError> {
        let result = Self {
            age: data.get_number("age").map(|x| x as i16),
            gender: RedCapGender::read(data),
            is_veteran: data.get_bool("is_veteran"),
            race: RedCapRace::read(data),
            ethnicity: data.get_enum("ethnicity"),
            health_insurance: data.get_enum_multi_select("insurance").unwrap_or_default(),
            highest_education_level: data.get_enum("education"),
            language: data.get_enum("language"),
        };

        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RedCapHealthOverview {
    /// Red Cap: height
    pub height: Option<i32>,
    /// Red Cap: health_conditions
    pub reported_health_conditions: Option<String>,
    /// Red Cap: allergies
    pub allergies: Option<String>,
    /// Red Cap: personal_cuff
    pub has_blood_pressure_cuff: Option<bool>,
    /// Red Cap: num_meds
    pub takes_more_than_5_medications: Option<bool>,
    /// Red Cap: mobility_devices
    pub mobility_devices: Option<Vec<MobilityDevice>>,
}

impl RedCapHealthOverview {
    pub async fn read<D: RedCapDataSet>(data: &D) -> Result<Self, RedCapConverterError> {
        let result = Self {
            height: data.get_number("height").map(|x| x as i32),
            reported_health_conditions: data.get_string("health_conditions"),
            allergies: data.get_string("allergies"),
            has_blood_pressure_cuff: data.get_bool("personal_cuff"),
            takes_more_than_5_medications: data.get_bool("num_meds"),
            mobility_devices: data.get_enum_multi_select("mobility_devices"),
        };

        Ok(result)
    }
}
