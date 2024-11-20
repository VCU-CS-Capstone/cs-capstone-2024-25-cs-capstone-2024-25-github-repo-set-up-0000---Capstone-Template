use std::{collections::HashMap, i32};

use sqlx::PgPool;
use tracing::{debug, error, info};

use crate::{
    database::red_cap::{
        case_notes::{CaseNote, CaseNoteHealthMeasures},
        participants::{
            goals::{ParticipantGoals, ParticipantGoalsSteps},
            health_overview::{HealthOverview, HealthOverviewType},
            NewDemographics, NewHealthOverview, NewParticipant, ParticipantDemograhics,
            ParticipantDemograhicsType, ParticipantMedications, ParticipantType, Participants,
        },
    },
    red_cap::{process_flat_json, MobilityDevice},
};

use super::{
    api::{ExportOptions, Forms, RedCapAPIError, RedcapClient},
    converter::{
        case_notes::{RedCapCaseNoteBase, RedCapHealthMeasures},
        goals::{RedCapGoals, RedCapGoalsSteps},
        medications::RedCapMedication,
        participants::{RedCapHealthOverview, RedCapParticipant, RedCapParticipantDemographics},
        RedCapConverter, RedCapConverterError,
    },
    flatten_data_to_red_cap_format,
};
#[derive(Debug, thiserror::Error)]
pub enum RedCapTaskError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    RedCapError(#[from] RedCapAPIError),
    #[error(transparent)]
    RedCapConversionError(#[from] RedCapConverterError),

    #[error("Participant not found")]
    ParticipantNotFound,

    #[error("Participant base information not pushed to red cap")]
    ParticipantBaseNotPushed,
}

pub async fn pull_record_base_types(
    record_id: i32,
    database: &PgPool,
    converter: &mut RedCapConverter,
    client: &RedcapClient,
) -> Result<(), RedCapTaskError> {
    let mut records = client
        .get_flat_json_forms(ExportOptions {
            forms: Some(vec![Forms::ParticipantInformation, Forms::HealthOverview].into()),
            records: Some(vec![record_id as usize].into()),

            ..Default::default()
        })
        .await?;
    let first_record = records.remove(0);
    let record = process_flat_json(first_record);

    let red_cap_participant = RedCapParticipant::read_participant(&record, converter).await?;
    let demographics = RedCapParticipantDemographics::read(&record).await?;
    let overview = RedCapHealthOverview::read(&record).await?;
    debug!(
        ?red_cap_participant,
        ?demographics,
        ?overview,
        "Read red cap data"
    );
    if let Some(mut participant) = Participants::find_by_red_cap_id(record_id, database).await? {
        info!(?participant, "Updating participant from red cap");
        participant
            .update_from_red_cap(red_cap_participant, demographics, overview, database)
            .await?;
    } else {
        let new_participant: NewParticipant = red_cap_participant.into();
        let new_demographics: Option<NewDemographics> = demographics.into();
        let new_overview: (NewHealthOverview, Option<Vec<MobilityDevice>>) = overview.into();

        let participant = new_participant.insert_return_participant(database).await?;

        if let Some(demographics) = new_demographics {
            demographics.insert_none(participant.id, database).await?;
        }

        let (overview, devices) = new_overview;
        if let Some(devices) = devices {
            let overview = overview
                .insert_return_health_overview(participant.id, database)
                .await?;
            for device in devices {
                overview.insert_mobility_device(device, database).await?;
            }
        } else {
            overview
                .insert_return_none(participant.id, database)
                .await?;
        }
    }

    Ok(())
}

pub async fn push_participant_to_red_cap(
    participant: i32,
    database: &PgPool,
    converter: &mut RedCapConverter,
    client: &RedcapClient,
) -> Result<(), RedCapTaskError> {
    let Some(mut participant) = Participants::find_by_id(participant, database).await? else {
        return Err(RedCapTaskError::ParticipantNotFound);
    };
    let demographics = ParticipantDemograhics::find_by_participant(participant.id, database)
        .await?
        .map(RedCapParticipantDemographics::from);
    let health_overview = HealthOverview::find_by_participant_id(participant.id, database)
        .await?
        .map(RedCapHealthOverview::from);
    let mut red_cap_participant = RedCapParticipant::from(participant.clone());
    let new_participant_id = if red_cap_participant.red_cap_id.is_none() {
        let next_id = client.get_next_record_id().await?;
        red_cap_participant.red_cap_id = Some(next_id);
        info!("Setting new red cap id to {}", next_id);
        Some(next_id)
    } else {
        None
    };
    let mut data = HashMap::new();
    red_cap_participant
        .write_to_data_set(&mut data, converter)
        .await?;
    if let Some(demographics) = demographics {
        demographics.write(&mut data);
    }
    if let Some(health_overview) = health_overview {
        health_overview.write(&mut data);
    }
    let flattened_data = flatten_data_to_red_cap_format(data);
    let records = vec![flattened_data];
    client.import_records(records).await?;

    if let Some(new_participant_id) = new_participant_id {
        participant
            .set_red_cap_id(Some(new_participant_id), database)
            .await?;
    }
    Ok(())
}
pub async fn push_participant_medications_to_red_cap(
    participant: i32,
    database: &PgPool,
    client: &RedcapClient,
) -> Result<(), RedCapTaskError> {
    let Some(participant) = Participants::find_by_id(participant, database).await? else {
        return Err(RedCapTaskError::ParticipantNotFound);
    };
    let Some(record_id) = participant.red_cap_id else {
        error!(?participant, "Participant does not have a red cap id");
        return Err(RedCapTaskError::ParticipantBaseNotPushed);
    };
    let medications =
        ParticipantMedications::get_all_participant_medications(participant.id, database).await?;
    let medications: Vec<RedCapMedication> = medications
        .into_iter()
        .filter_map(|x| {
            if x.red_cap_index.is_none() {
                error!("Medication does not have a red cap index");
                return None;
            }
            Some(x.into())
        })
        .collect::<Vec<_>>();

    let mut data = std::collections::HashMap::new();

    for medication in medications {
        medication.write(&mut data);
    }
    data.insert("record_id".into(), record_id.into());

    let flattened_data = super::flatten_data_to_red_cap_format(data);
    let records = vec![flattened_data];

    client.import_records(records).await?;
    Ok(())
}
pub async fn push_participant_goals_to_red_cap(
    participant: i32,
    database: &PgPool,
    converter: &mut RedCapConverter,
    client: &RedcapClient,
) -> Result<(), RedCapTaskError> {
    let Some(participant) = Participants::find_by_id(participant, database).await? else {
        return Err(RedCapTaskError::ParticipantNotFound);
    };
    let Some(record_id) = participant.red_cap_id else {
        error!(?participant, "Participant does not have a red cap id");
        return Err(RedCapTaskError::ParticipantBaseNotPushed);
    };
    let goals = ParticipantGoals::get_all_participant_goals(participant.id, database).await?;
    let goals: Vec<RedCapGoals> = goals
        .into_iter()
        .filter_map(|x| {
            if x.red_cap_index.is_none() {
                error!("Medication does not have a red cap index");
                return None;
            }
            Some(x.into())
        })
        .collect::<Vec<_>>();

    let goal_steps =
        ParticipantGoalsSteps::get_all_participant_goals_steps(participant.id, database).await?;
    let mut red_cap_goal_steps = Vec::with_capacity(goal_steps.capacity());
    for goal_step in goal_steps {
        let red_cap_goal_step = RedCapGoalsSteps::from_db(goal_step, converter).await?;
        red_cap_goal_steps.push(red_cap_goal_step);
    }

    let mut data = std::collections::HashMap::new();

    for goal in goals {
        goal.write(&mut data);
    }
    for goal_step in red_cap_goal_steps {
        goal_step.write(&mut data);
    }
    data.insert("record_id".into(), record_id.into());

    let flattened_data = super::flatten_data_to_red_cap_format(data);
    let records = vec![flattened_data];

    client.import_records(records).await?;
    Ok(())
}
pub async fn push_case_notes_to_redcap(
    participant: i32,
    database: &PgPool,
    converter: &mut RedCapConverter,
    client: &RedcapClient,
) -> Result<(), RedCapTaskError> {
    let Some(participant) = Participants::find_by_id(participant, database).await? else {
        error!("Participant not found");
        return Err(RedCapTaskError::ParticipantNotFound);
    };
    let Some(record_id) = participant.red_cap_id else {
        error!("Participant does not have a red cap id");
        return Err(RedCapTaskError::ParticipantBaseNotPushed);
    };
    let mut case_notes = CaseNote::find_by_participant_id(participant.id, database)
        .await
        .unwrap_or_default();
    case_notes.sort_by(|a, b| {
        a.redcap_instance
            .unwrap_or(i32::MAX)
            .cmp(&b.redcap_instance.unwrap_or(i32::MAX))
    });

    for (index, case_note) in case_notes.into_iter().enumerate() {
        let instance_id = case_note.redcap_instance.as_ref();
        debug!(?instance_id, ?index, "Processing case note");
        let mut data = std::collections::HashMap::new();
        let red_cap_case_note: RedCapCaseNoteBase = case_note.clone().into();

        let health_measures: Option<RedCapHealthMeasures> =
            CaseNoteHealthMeasures::find_by_case_note_id(case_note.id, database)
                .await?
                .map(|x| x.into());
        red_cap_case_note
            .write_case_note(&mut data, converter)
            .await?;

        if let Some(health_measures) = health_measures {
            health_measures.write_health_measures(&mut data);
        }
        data.insert("record_id".into(), record_id.into());
        println!("{:?}", data);
        let flattened_data = super::flatten_data_to_red_cap_format(data);
        let records = vec![flattened_data];
        client.import_records(records).await?;

        if case_note.redcap_instance.is_none() {
            case_note
                .update_instance_id(index as i32 + 1, database)
                .await?;
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {

    use crate::red_cap::{api::RedcapClient, converter::RedCapConverter};
    #[tokio::test]
    #[ignore]
    pub async fn import_record_one_from_redcap() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();

        let database = crate::database::tests::setup_red_cap_db_test(&env).await?;
        let mut converter: RedCapConverter = RedCapConverter::new(database.clone()).await?;
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());

        super::pull_record_base_types(1, &database, &mut converter, &client).await?;
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    pub async fn import_record_to_red_cap() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();

        let database = crate::database::tests::setup_red_cap_db_test(&env).await?;
        let mut converter: RedCapConverter = RedCapConverter::new(database.clone()).await?;
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());

        super::push_participant_to_red_cap(1, &database, &mut converter, &client).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    pub async fn import_record_medications() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();

        let database = crate::database::tests::setup_red_cap_db_test(&env).await?;
        //let mut converter = RedCapConverter::new(database.clone()).await?;
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());

        super::push_participant_medications_to_red_cap(1, &database, &client).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    pub async fn import_record_goals() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();

        let database = crate::database::tests::setup_red_cap_db_test(&env).await?;
        let mut converter: RedCapConverter = RedCapConverter::new(database.clone()).await?;

        //let mut converter = RedCapConverter::new(database.clone()).await?;
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());

        super::push_participant_goals_to_red_cap(1, &database, &mut converter, &client).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    pub async fn import_record_case_notes() -> anyhow::Result<()> {
        let env = crate::env_utils::read_env_file_in_core("test.env").unwrap();
        crate::test_utils::init_logger();

        let database = crate::database::tests::setup_red_cap_db_test(&env).await?;
        let mut converter: RedCapConverter = RedCapConverter::new(database.clone()).await?;

        //let mut converter = RedCapConverter::new(database.clone()).await?;
        let client = RedcapClient::new(env.get("RED_CAP_TOKEN").unwrap().to_owned());

        super::push_case_notes_to_redcap(1, &database, &mut converter, &client).await?;
        Ok(())
    }
}
