use std::{collections::HashMap, num::ParseIntError};

use thiserror::Error;
use tracing::instrument;
mod request;
pub use request::*;
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
    pub async fn get_flat_json_forms(
        &self,
        ExportOptions {
            forms,
            records,
            fields,
        }: ExportOptions,
    ) -> Result<Vec<HashMap<String, String>>, RedCapAPIError> {
        let forms_as_string = forms.map(|forms| forms.to_string());
        let records_as_string = records.map(|record| record.to_string());
        let fields_as_string = fields.map(|fields| fields.to_string());
        let mut map = HashMap::new();
        map.insert("content", "record");
        map.insert("token", &self.token);
        map.insert("action", "export");
        map.insert("format", Format::Json.as_ref());
        map.insert("type", FormatType::Flat.as_ref());
        if let Some(fields) = fields_as_string.as_deref() {
            map.insert("fields", fields);
        }
        if let Some(forms) = forms_as_string.as_deref() {
            map.insert("forms", forms);
        }
        if let Some(records) = records_as_string.as_deref() {
            map.insert("records", records);
        }
        let request = self
            .client
            .post("https://redcap.vcu.edu/api/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&map)
            .build()?;

        let response = self.client.execute(request).await?;
        let response = response.text().await?;

        let records: Vec<HashMap<String, String>> = serde_json::from_str(&response)?;
        Ok(records)
    }
}
#[cfg(test)]
mod tests {
    use crate::red_cap::{
        api::{ExportOptions, Forms, RedcapClient},
        converter::{
            goals::RedCapCompleteGoals,
            medications::RedCapMedication,
            participants::{
                RedCapHealthOverview, RedCapParticipant, RedCapParticipantDemographics,
            },
            RedCapConverter,
        },
        process_flat_json,
    };

    #[tokio::test]
    #[ignore]
    pub async fn get_all_record_ids() {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());
        let records = client
            .get_flat_json_forms(ExportOptions {
                fields: Some(vec!["record_id".to_string()].into()),
                ..Default::default()
            })
            .await
            .unwrap();
        println!("{:#?}", records);
    }

    #[tokio::test]
    #[ignore]
    pub async fn get_base_forms_for_id_1() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();

        let database = crate::database::tests::setup_red_cap_db_test(&env).await?;
        let mut converter = RedCapConverter::new(database).await?;
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());
        let records = client
            .get_flat_json_forms(ExportOptions {
                forms: Some(vec![Forms::ParticipantInformation, Forms::HealthOverview].into()),
                records: Some(vec![1].into()),

                ..Default::default()
            })
            .await
            .unwrap();

        for record in records {
            let record = process_flat_json(record);
            println!("{:#?}", record);
            let red_cap_participant =
                RedCapParticipant::read_participant(&record, &mut converter).await?;
            println!("{:#?}", red_cap_participant);
            let demographics = RedCapParticipantDemographics::read(&record).await?;

            println!("{:#?}", demographics);
            let overview = RedCapHealthOverview::read(&record).await?;

            println!("{:#?}", overview);
        }
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    pub async fn get_case_notes_for_id_1() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());
        let records = client
            .get_flat_json_forms(ExportOptions {
                forms: Some(vec![Forms::CaseNotes].into()),
                records: Some(vec![1].into()),

                ..Default::default()
            })
            .await
            .unwrap();

        for record in records {
            let record = process_flat_json(record);
            println!("{:#?}", record);
        }
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    pub async fn get_goals_for_id_1() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());
        let records = client
            .get_flat_json_forms(ExportOptions {
                forms: Some(vec![Forms::WellnessGoals].into()),
                records: Some(vec![1].into()),

                ..Default::default()
            })
            .await
            .unwrap();

        for record in records {
            let record = process_flat_json(record);
            println!("{:#?}", record);
            let goals = RedCapCompleteGoals::read(&record).await?;
            println!("{:#?}", goals);
        }
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    pub async fn get_medications_for_id_1() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());
        let records = client
            .get_flat_json_forms(ExportOptions {
                forms: Some(vec![Forms::Medications].into()),
                records: Some(vec![1].into()),

                ..Default::default()
            })
            .await
            .unwrap();

        for record in records {
            let record = process_flat_json(record);
            println!("{:#?}", record);
            let goals = RedCapMedication::read(&record);
            println!("{:#?}", goals);
        }
        Ok(())
    }
}
