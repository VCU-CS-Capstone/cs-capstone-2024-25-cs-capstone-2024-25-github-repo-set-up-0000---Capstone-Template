use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::red_cap_data::MobilityDevice;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct HealthOverviw {
    pub id: i32,
    /// 1:1 with [super::Participants]
    pub participant_id: i32,
    /// Red Cap: height
    pub height: Option<i32>,
    /// Red Cap: health_conditions
    pub reported_health_conditions: Option<String>,
    /// Red Cap: allergies
    pub allergies: Option<String>,
    /// Red Cap: personal_cuff
    pub has_blood_pressure_cuff: Option<bool>,
    /// Red Cap: num_meds
    pub takes_more_than_5_medications: Option<bool>,
}
impl HealthOverviw {
    pub async fn get_mobility_devices(
        &self,
        database: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    ) -> sqlx::Result<Vec<HealthOverviewMobilityDevices>> {
        sqlx::query_as(
            r#"
            SELECT * FROM health_overview_mobility_devices
            WHERE health_overview_id = $1
            "#,
        )
        .bind(self.id)
        .fetch_all(database)
        .await
    }
    pub async fn insert_mobility_device(
        &self,
        device: MobilityDevice,
        database: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    ) -> sqlx::Result<()> {
        sqlx::query(
            r#"
                INSERT INTO health_overview_mobility_devices (health_overview_id, mobility_devices)
                VALUES ($1, $2)
                "#,
        )
        .bind(self.id)
        .bind(device)
        .execute(database)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct HealthOverviewMobilityDevices {
    pub id: i32,
    /// 1:many with [HealthOverviw]
    pub health_overview_id: i32,
    /// Red Cap: info_mobility
    pub device: MobilityDevice,
}
