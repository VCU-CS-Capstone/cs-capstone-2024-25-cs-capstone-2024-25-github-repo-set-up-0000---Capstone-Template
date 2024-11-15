use crate::database::prelude::*;
use cs25_303_macros::Columns;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use tracing::debug;

use crate::red_cap_data::MobilityDevice;
pub trait HealthOverviewType: for<'r> FromRow<'r, PgRow> + Unpin + Send + Sync {
    fn get_id(&self) -> i32;

    fn columns() -> Vec<HealthOverviewColumn> {
        HealthOverviewColumn::all()
    }

    async fn find_by_id(
        id: i32,
        database: impl Executor<'_, Database = Postgres>,
    ) -> sqlx::Result<Option<Self>> {
        let columns = concat_columns(&Self::columns(), None);
        let result = sqlx::query_as(&format!(
            "SELECT {columns} FROM health_overview WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(database)
        .await?;
        Ok(result)
    }

    async fn find_by_participant_id(
        participant_id: i32,
        database: impl Executor<'_, Database = Postgres>,
    ) -> sqlx::Result<Option<Self>> {
        let mut result =
            SimpleSelectQueryBuilder::new("participant_health_overview", &Self::columns());
        result.where_equals(HealthOverviewColumn::ParticipantId, participant_id);
        if tracing::enabled!(tracing::Level::DEBUG) {
            let query = result.sql();
            debug!(?query, "Executing Query");
        }
        let result = result.query_as::<Self>().fetch_optional(database).await?;
        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Columns)]
pub struct HealthOverview {
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
impl HealthOverviewType for HealthOverview {
    fn get_id(&self) -> i32 {
        self.id
    }
}
impl HealthOverview {
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
