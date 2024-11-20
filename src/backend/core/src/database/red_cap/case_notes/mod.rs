pub mod new;
use crate::database::prelude::*;
use crate::{
    database::tools::{SimpleSelectQueryBuilder, TableType},
    red_cap::VisitType,
};
use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
pub mod questions;
pub mod staff;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Columns)]
pub struct CaseNote {
    pub id: i32,
    /// Relates to #[crate::database::red_cap::participants::Participants]
    pub participant_id: i32,
    /// Relates to #[crate::database::red_cap::Locations]
    ///
    /// RWHP Red Cap ID: `rhwp_location_visit`
    /// MHWP Red Cap ID: `mhwp_location_visit`
    ///
    /// Petersburg Sub Red Cap ID: `mhwp_location_visit_petersburg`
    pub location: Option<i32>,
    /// Red Cap ID: `visit_type`
    pub visit_type: Option<VisitType>,
    /// Redcap ID: exit_age
    pub age: Option<i16>,
    /// Red Cap ID: `reason`
    pub reason_for_visit: Option<String>,
    /// Red Cap ID: subjective_info
    pub info_provided_by_caregiver: Option<String>,
    /// Red Cap ID: visit_date
    pub date_of_visit: NaiveDate,
    /// DATABASE ONLY
    pub pushed_to_redcap: bool,

    pub completed: bool,

    /// Instance Number of the case note
    pub redcap_instance: Option<i32>,
    /// DATABASE ONLY
    pub last_synced_with_redcap: Option<DateTime<FixedOffset>>,
}
impl TableType for CaseNote {
    type Columns = CaseNoteColumn;
    fn table_name() -> &'static str {
        "case_notes"
    }
}
impl CaseNote {
    pub async fn find_by_id(id: i32, database: &sqlx::PgPool) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query_as(
            "
            SELECT * FROM case_notes
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(database)
        .await?;
        Ok(result)
    }
    pub async fn find_by_participant_id(
        participant_id: i32,
        database: &sqlx::PgPool,
    ) -> sqlx::Result<Vec<Self>> {
        let result = sqlx::query_as(
            "
            SELECT * FROM case_notes
            WHERE participant_id = $1
            ",
        )
        .bind(participant_id)
        .fetch_all(database)
        .await?;
        Ok(result)
    }
    pub async fn update_instance_id(
        &self,
        instance_id: i32,
        database: &sqlx::PgPool,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "
            UPDATE case_notes
            SET redcap_instance = $1
            WHERE id = $2
            ",
        )
        .bind(instance_id)
        .bind(self.id)
        .execute(database)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Columns)]
pub struct CaseNoteHealthMeasures {
    pub id: i32,
    /// 1:1 with [CaseNote]
    pub case_note_id: i32,
    /// Red Cap ID bp_sit_syst
    ///
    /// Must Exist if blood_pressure_sit_diastolic exists
    pub blood_pressure_sit_systolic: Option<i16>,
    ///Red Cap ID bp_sit_dia
    ///
    /// Must Exist if blood_pressure_sit_systolic exists
    pub blood_pressure_sit_diastolic: Option<i16>,
    /// Red Cap ID bp_stand_syst
    ///
    /// Must Exist if blood_pressure_stand_diastolic exists
    pub blood_pressure_stand_systolic: Option<i16>,
    /// Red Cap ID bp_stand_dia
    ///
    /// Must Exist if blood_pressure_stand_systolic exists
    pub blood_pressure_stand_diastolic: Option<i16>,
    /// Weight Taken RED Cap ID: weight_yn
    /// Weight Red Cap: weight
    pub weight: Option<f32>,
    /// Redcap ID: glucose_yn
    pub glucose_tested: bool,
    /// Redcap ID: glucose
    pub glucose_result: Option<f32>,
    /// Redcap ID: glucose_fasting
    pub fasted_atleast_2_hours: bool,
    ///Function, Assistive Devices, and/or Limitations to ADLs/IADLs
    /// Redcap ID: visit_function
    pub other: Option<String>,
}
impl TableType for CaseNoteHealthMeasures {
    type Columns = CaseNoteHealthMeasuresColumn;
    fn table_name() -> &'static str {
        "case_note_health_measures"
    }
}
impl CaseNoteHealthMeasures {
    pub async fn find_by_id(id: i32, database: &sqlx::PgPool) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query_as(
            "
            SELECT * FROM case_note_health_measures
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(database)
        .await?;
        Ok(result)
    }
    pub async fn find_by_case_note_id(
        case_note_id: i32,
        database: &sqlx::PgPool,
    ) -> sqlx::Result<Option<Self>> {
        SimpleSelectQueryBuilder::new(Self::table_name(), &CaseNoteHealthMeasuresColumn::all())
            .where_equals(CaseNoteHealthMeasuresColumn::CaseNoteId, case_note_id)
            .query_as()
            .fetch_optional(database)
            .await
    }
}
/// Case Note Questions Related to a patient call to 911 or their PCP
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteOtherHealthVisits {
    pub id: i32,
    /// 1:1 with [CaseNote]
    pub case_note_id: i32,
    /// Red Cap ID: `ercall`
    ///
    /// The two fields are derived from
    /// - No - not required
    /// - Yes, and taken to hospital in ambulance
    /// - Yes, but refused ambulance
    pub emergency_number_called: bool,

    pub refused_ambulance: Option<bool>,
    /// Red Cap ID: `erreason`
    pub reason_for_call: Option<String>,
    ///Red Cap ID: exit_pcp_visit
    pub last_see_pcp: Option<String>,
    /// Red Cap ID: transition_hospital
    pub did_visit_hospital: Option<bool>,
    /// Red Cap ID: `hospital_transition`
    pub hospital_visit: Option<String>,
    /// Red Cap ID: `transition_ed`
    pub did_visit_ed: Option<bool>,
    /// Red Cap ID: `ed_transition`
    pub ed_visit: Option<String>,
}
