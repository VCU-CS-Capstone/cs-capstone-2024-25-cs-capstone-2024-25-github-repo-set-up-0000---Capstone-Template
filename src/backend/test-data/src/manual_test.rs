use std::path::PathBuf;

use cs25_303_core::database::red_cap::participants::{
    goals::{NewParticipantGoal, NewParticipantGoalsSteps},
    NewDemographics, NewHealthOverview, NewMedication, NewParticipant,
};
use sqlx::{Postgres, Transaction};
use tracing::info;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TestParticipantFile {
    pub participant: NewParticipant,
    pub demographics: Option<NewDemographics>,
    pub health_overview: Option<NewHealthOverview>,
    #[serde(default)]
    pub medications: Vec<NewMedication>,
}
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GoalFile {
    pub goal: NewParticipantGoal,
    #[serde(default)]
    pub steps: Vec<NewParticipantGoalsSteps>,
}
pub async fn create_participant_from_dir(
    path: PathBuf,
    database: &mut Transaction<'static, Postgres>,
) -> anyhow::Result<()> {
    let participant_file = path.join("participant.json");
    if !participant_file.exists() {
        return Err(anyhow::anyhow!("No participant file found"));
    }
    info!("Creating participant from: {:?}", participant_file);
    let participant = std::fs::read_to_string(participant_file)?;
    info!("Read File Parsing it");
    let participant: TestParticipantFile = serde_json::from_str(&participant)?;
    let TestParticipantFile {
        participant,
        demographics,
        health_overview,
        medications,
    } = participant;

    let participant = participant
        .insert_return_participant(database.as_mut())
        .await?;
    if let Some(demographics) = demographics {
        demographics
            .insert_none(participant.id, database.as_mut())
            .await?;
    }
    if let Some(health_overview) = health_overview {
        health_overview
            .insert_return_none(participant.id, database.as_mut())
            .await?;
    }
    for medication in medications {
        medication
            .insert_return_none(participant.id, database.as_mut())
            .await?;
    }
    find_and_add_goals(participant.id, path, database).await?;
    // TODO: Read Goals and Case Notes
    Ok(())
}

async fn find_and_add_goals(
    participant: i32,
    path: PathBuf,
    database: &mut Transaction<'static, Postgres>,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if super::does_file_name_start_with(&path, "goal")? {
            let goal_file = std::fs::read_to_string(path)?;
            let goal_file: GoalFile = serde_json::from_str(&goal_file)?;
            let GoalFile { goal, steps } = goal_file;
            let goal = goal
                .insert_return_goal(participant, database.as_mut())
                .await?;
            for step in steps {
                step.insert_with_goal_return_none(participant, goal.id, database.as_mut())
                    .await?;
            }
        }
    }
    Ok(())
}
