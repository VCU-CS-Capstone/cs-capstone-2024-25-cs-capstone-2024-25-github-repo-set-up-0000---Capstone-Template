use serde::{Deserialize, Serialize};

use crate::database::DBTime;

/// Table: health_overview
/// Redcap Entry: TODO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthOverview {
    pub id: i32,
    pub height: f32,
    pub participant_id: i32,

    pub pulled_from_redcap_last: Option<DBTime>,

    pub last_updated: DBTime,
}
