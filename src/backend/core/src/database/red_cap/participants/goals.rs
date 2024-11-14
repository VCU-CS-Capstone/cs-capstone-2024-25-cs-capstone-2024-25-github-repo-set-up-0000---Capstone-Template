//!
//!
//! So Umm instead of using the repeating entry stuff in RedCap
//! They have a form that is up to 10 goals. Where each one could be null
//!
//! So yeah. We are just going to use a 1:many relationship
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Executor};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewParticipantGoal {
    pub goal: String,
    pub is_active: Option<bool>,
}
impl NewParticipantGoal {
    pub async fn insert_return_goal(
        self,
        participant_id: i32,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> sqlx::Result<ParticipantGoals> {
        let Self { goal, is_active } = self;
        sqlx::query_as(
            r#"
            INSERT INTO participant_goals (participant_id, goal, is_active)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(participant_id)
        .bind(goal)
        .bind(is_active)
        .fetch_one(database)
        .await
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ParticipantGoals {
    pub id: i32,
    pub participant_id: i32,
    pub goal: String,
    pub is_active: Option<bool>,
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
    pub action_step: bool,
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
            ..
        } = self;
        sqlx::query(
            r#"
            INSERT INTO participant_goal_steps (
            participant_id,
             goal_id, step,
             confidence_level,
             date_set,
             date_to_be_completed,
             action_step
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(participant_id)
        .bind(related_goal_id)
        .bind(step)
        .bind(confidence_level)
        .bind(date_set)
        .bind(date_to_be_completed)
        .bind(action_step)
        .execute(database)
        .await?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
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
}
