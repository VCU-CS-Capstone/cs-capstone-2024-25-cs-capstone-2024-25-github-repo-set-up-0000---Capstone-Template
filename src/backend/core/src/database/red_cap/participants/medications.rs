use crate::database::prelude::*;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::red_cap::MedicationFrequency;

use super::TableType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Columns)]
pub struct ParticipantMedications {
    pub id: i32,
    pub participant_id: i32,
    pub name: String,
    pub dosage: String,
    pub frequency: Option<MedicationFrequency>,
    pub date_prescribed: Option<NaiveDate>,
    pub date_entered_into_system: NaiveDate,
    pub is_current: Option<bool>,
    pub date_discontinued: Option<NaiveDate>,
    pub comments: Option<String>,
    pub red_cap_index: Option<i32>,
}
impl TableType for ParticipantMedications {
    type Columns = ParticipantMedicationsColumn;
    fn table_name() -> &'static str {
        "participant_medications"
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NewMedication {
    pub name: String,
    pub dosage: Option<String>,
    pub frequency: Option<MedicationFrequency>,
    pub date_prescribed: Option<chrono::NaiveDate>,
    pub date_entered_into_system: Option<NaiveDate>,
    pub is_current: Option<bool>,
    pub date_discontinued: Option<chrono::NaiveDate>,
    pub comments: Option<String>,
    pub red_cap_index: Option<i32>,
}
impl NewMedication {
    pub async fn insert_return_none(
        self,
        participant_id: i32,
        database: impl Executor<'_, Database = sqlx::Postgres>,
    ) -> DBResult<()> {
        let Self {
            name,
            dosage,
            frequency,
            date_prescribed,
            date_entered_into_system,
            is_current,
            date_discontinued,
            comments,
            red_cap_index,
        } = self;

        let date_entered_into_system =
            date_entered_into_system.unwrap_or_else(|| Local::now().date_naive());
        SimpleInsertQueryBuilder::new(ParticipantMedications::table_name())
            .insert(ParticipantMedicationsColumn::ParticipantId, participant_id)
            .insert(ParticipantMedicationsColumn::Name, name)
            .insert(ParticipantMedicationsColumn::Dosage, dosage)
            .insert(ParticipantMedicationsColumn::Frequency, frequency)
            .insert(
                ParticipantMedicationsColumn::DatePrescribed,
                date_prescribed,
            )
            .insert(
                ParticipantMedicationsColumn::DateEnteredIntoSystem,
                date_entered_into_system,
            )
            .insert(ParticipantMedicationsColumn::IsCurrent, is_current)
            .insert(
                ParticipantMedicationsColumn::DateDiscontinued,
                date_discontinued,
            )
            .insert(ParticipantMedicationsColumn::Comments, comments)
            .insert(ParticipantMedicationsColumn::RedCapIndex, red_cap_index)
            .query()
            .execute(database)
            .await?;
        Ok(())
    }
}
