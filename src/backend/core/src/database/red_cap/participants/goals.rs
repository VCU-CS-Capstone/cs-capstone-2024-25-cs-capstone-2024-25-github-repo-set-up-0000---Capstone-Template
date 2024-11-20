//!
//!
//! So Umm instead of using the repeating entry stuff in RedCap
//! They have a form that is up to 10 goals. Where each one could be null
//!
//! So yeah. We are just going to use a 1:many relationship
use crate::database::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Executor};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NewParticipantGoal {
    pub goal: String,
    pub is_active: Option<bool>,
    pub red_cap_index: Option<i32>,
}
impl NewParticipantGoal {
    pub async fn insert_return_goal(
        self,
        participant_id: i32,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> sqlx::Result<ParticipantGoals> {
        let Self {
            goal,
            is_active,
            red_cap_index,
        } = self;
        SimpleInsertQueryBuilder::new(ParticipantGoals::table_name())
            .insert(ParticipantGoalsColumn::ParticipantId, participant_id)
            .insert(ParticipantGoalsColumn::Goal, goal)
            .insert(ParticipantGoalsColumn::IsActive, is_active)
            .insert(ParticipantGoalsColumn::RedCapIndex, red_cap_index)
            .return_all()
            .query_as()
            .fetch_one(database)
            .await
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Columns)]
pub struct ParticipantGoals {
    pub id: i32,
    pub participant_id: i32,
    pub goal: String,
    pub is_active: Option<bool>,
    pub red_cap_index: Option<i32>,
}
impl TableType for ParticipantGoals {
    type Columns = ParticipantGoalsColumn;
    fn table_name() -> &'static str {
        "participant_goals"
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NewParticipantGoalsSteps {
    pub goal_id: Option<i32>,
    pub step: String,
    pub confidence_level: Option<i16>,
    pub date_set: Option<NaiveDate>,
    pub date_to_be_completed: Option<NaiveDate>,
    /// True == Yes
    /// False == No
    /// Select No until goal is achieved
    #[serde(default)]
    pub action_step: Option<bool>,
    pub red_cap_index: Option<i32>,
}
impl NewParticipantGoalsSteps {
    pub async fn insert_with_goal_return_none(
        self,
        participant_id: i32,
        related_goal_id: i32,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> sqlx::Result<()> {
        let Self {
            step,
            confidence_level,
            date_set,
            date_to_be_completed,
            action_step,
            red_cap_index,
            ..
        } = self;
        SimpleInsertQueryBuilder::new(ParticipantGoalsSteps::table_name())
            .insert(ParticipantGoalsStepsColumn::ParticipantId, participant_id)
            .insert(ParticipantGoalsStepsColumn::GoalId, related_goal_id)
            .insert(ParticipantGoalsStepsColumn::Step, step)
            .insert(
                ParticipantGoalsStepsColumn::ConfidenceLevel,
                confidence_level,
            )
            .insert(ParticipantGoalsStepsColumn::DateSet, date_set)
            .insert(
                ParticipantGoalsStepsColumn::DateToBeCompleted,
                date_to_be_completed,
            )
            .insert(ParticipantGoalsStepsColumn::ActionStep, Some(action_step))
            .insert(ParticipantGoalsStepsColumn::RedCapIndex, red_cap_index)
            .query()
            .execute(database)
            .await?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Columns)]
pub struct ParticipantGoalsSteps {
    pub id: i32,
    pub goal_id: Option<i32>,
    pub participant_id: i32,
    /// This is technically nullable. But if it is null. I will not put it in the database
    pub step: String,
    pub confidence_level: Option<i16>,
    pub date_set: Option<NaiveDate>,
    pub date_to_be_completed: Option<NaiveDate>,
    /// True == Yes
    /// False == No
    /// Select No until goal is achieved
    pub action_step: Option<bool>,
    pub red_cap_index: Option<i32>,
}
impl TableType for ParticipantGoalsSteps {
    type Columns = ParticipantGoalsStepsColumn;
    fn table_name() -> &'static str {
        "participant_goal_steps"
    }
}
