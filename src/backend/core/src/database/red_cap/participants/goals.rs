//!
//!
//! So Umm instead of using the repeating entry stuff in RedCap
//! They have a form that is up to 10 goals. Where each one could be null
//!
//! So yeah. We are just going to use a 1:many relationship
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ParticipantGoals {
    pub id: i64,
    pub participant_id: i64,
    pub goal: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ParticipantGoalsSteps {
    pub id: i64,
    pub related_goal_id: Option<i64>,
    pub participant_id: i64,
    /// This is technically nullable. But if it is null. I will not put it in the database
    pub step: String,
    pub confidence_in_achieving: Option<i16>,
    pub date_set: Option<NaiveDate>,
    pub date_to_be_achieved: Option<NaiveDate>,
    /// True == Yes
    /// False == No
    /// Select No until goal is achieved
    pub action_step: bool,
}
