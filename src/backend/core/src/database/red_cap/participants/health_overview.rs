use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::red_cap_data::MobilityDevice;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct HealthOverviw {
    pub id: i64,
    /// 1:1 with [super::Participants]
    pub participant_id: i64,
    /// Red Cap: height
    pub height: Option<i32>,
    /// Red Cap: health_conditions
    pub reported_health_conditions: Option<String>,
    /// Red Cap: allergies
    pub allergies: Option<String>,
    /// Red Cap: info_mobility
    pub mobility_devices: Option<Vec<MobilityDevice>>,
    /// Red Cap: personal_cuff
    pub has_blood_pressure_cuff: Option<bool>,
    /// Red Cap: num_meds
    pub takes_more_than_5_medications: Option<bool>,
}
