//! Extra Tables of Info for Participants
//!
//! These tables still need to be implemented
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ParticipantEmergencyContact {
    pub id: i64,
    pub participant_id: i64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ParticipantHealthCareProviders {
    pub id: i64,
    pub participant_id: i64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ParticipantPharmacy {
    pub id: i64,
    pub participant_id: i64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ParticipantAdvancedCarePlan {
    pub id: i64,
    pub participant_id: i64,
}
