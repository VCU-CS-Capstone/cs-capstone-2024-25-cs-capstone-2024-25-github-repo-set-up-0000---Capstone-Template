use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::red_cap_data::Programs;

/// Red Cap ID: pilot_gaps_coordination
/// # Values for RHWP
/// - Church Hill House
/// - Dominion Place
/// - Highland Park
/// - 4th Ave
/// - Health Hub
/// - The Rosa
/// # Values for MHWP
/// - Lawrenceville
/// - Petersburg
/// - Tappahannock
/// - Southwood
///
/// TODO: Petersburg has sub locations.
/// # Values for Petersburg
///- VCRC
///- Police substation
///- Gilhaven
///- VSU Van
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Locations {
    pub id: i32,
    pub name: String,
    pub program: Programs,
    pub parent_location: Option<i32>,
}
