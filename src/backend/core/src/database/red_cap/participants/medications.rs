use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::red_cap_data::MedicationFrequency;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ParticipantMedications {
    pub id: i32,
    pub participant_id: i32,
    pub name: String,
    pub dosage: String,
    pub frequency: MedicationFrequency,
    pub date_prescribed: Option<chrono::NaiveDate>,
    pub date_entered_into_system: NaiveDate,
    pub is_current: Option<bool>,
    pub date_discontinued: Option<chrono::NaiveDate>,
    pub comments: Option<String>,
}
