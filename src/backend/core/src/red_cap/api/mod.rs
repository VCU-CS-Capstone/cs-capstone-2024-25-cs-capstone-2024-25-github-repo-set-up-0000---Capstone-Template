use std::{collections::HashMap, num::ParseIntError};

use responses::Record;
use strum::{Display, EnumString};
use thiserror::Error;
use tracing::instrument;

pub mod responses;
pub mod utils;
#[derive(Debug, Error)]
pub enum RedCapParseError {
    #[error("Invalid multi checkbox field: {input:?}, reason: {reason:?}")]
    InvalidMultiCheckboxField { input: String, reason: GenericError },
    #[error("Missing field: {field:?}")]
    MissingField { field: String },
}
#[derive(Debug, Error)]
pub enum GenericError {
    #[error(transparent)]
    ParseNumber(#[from] ParseIntError),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum RedCapAPIError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Parse(#[from] serde_json::Error),
}
#[derive(Debug, EnumString, Display)]
pub enum Forms {
    #[strum(serialize = "participant_information")]
    ParticipantInformation,
    #[strum(serialize = "health_overview")]
    HealthOverview,
    #[strum(serialize = "medications")]
    Medications,
    #[strum(serialize = "wellness_goals")]
    WellnessGoals,
    #[strum(serialize = "case_note")]
    CaseNotes,
}

#[derive(Debug)]
pub struct RedcapClient {
    pub token: String,
    pub client: reqwest::Client,
}
impl RedcapClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::default(),
        }
    }
    #[instrument]
    pub async fn get_forms_for_record(
        &self,
        record: usize,
        forms: &[Forms],
    ) -> Result<Vec<Record>, RedCapAPIError> {
        let forms_as_string = forms
            .iter()
            .map(|form| form.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let record_as_string = record.to_string();
        let mut map = HashMap::new();
        map.insert("content", "record");
        map.insert("token", &self.token);
        map.insert("action", "export");
        map.insert("format", "json");
        map.insert("type", "eav");
        map.insert("forms", &forms_as_string);
        map.insert("records", &record_as_string);
        let request = self
            .client
            .post("https://redcap.vcu.edu/api/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&map)
            .build()?;

        let response = self.client.execute(request).await?;
        let response = response.text().await?;

        let records: Vec<Record> = serde_json::from_str(&response)?;
        println!("{}", response);
        Ok(records)
    }
}
#[cfg(test)]
mod tests {
    use crate::red_cap::api::{Forms, RedcapClient};

    #[tokio::test]
    #[ignore]
    pub async fn get_all_records() {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());
        let records = client
            .get_forms_for_record(
                1,
                &[
                    Forms::ParticipantInformation,
                    Forms::HealthOverview,
                    Forms::Medications,
                    Forms::WellnessGoals,
                    Forms::CaseNotes,
                ],
            )
            .await
            .unwrap();
        println!("{:#?}", records);
    }
}
