use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::red_cap_data::MedicationFrequency;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ParticipantMedications {
    pub id: i64,
    pub participant_id: i64,
    pub medication_name: String,
    pub dosage: String,
    pub frequency: MedicationFrequency,
    pub date_prescribed: Option<chrono::NaiveDate>,
    pub date_entered_into_system: chrono::NaiveDateTime,
    pub is_current: Option<bool>,
    pub date_discontinued: Option<chrono::NaiveDate>,
    pub comments: Option<String>,
}
