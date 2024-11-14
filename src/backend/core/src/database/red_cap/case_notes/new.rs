use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{database::DBResult, red_cap_data::VisitType};

use super::CaseNote;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewCaseNote {
    pub location: Option<i32>,
    pub visit_type: Option<VisitType>,
    pub age: i16,
    pub reason_for_visit: Option<String>,
    pub info_provided_by_caregiver: Option<String>,
    pub date_of_visit: NaiveDate,
    pub pushed_to_redcap: bool,
    pub redcap_instance: Option<i32>,
}
impl NewCaseNote {
    pub async fn insert_return_case_note(
        self,
        participant: i32,
        database: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    ) -> DBResult<CaseNote> {
        let Self {
            location,
            visit_type,
            age,
            reason_for_visit,
            info_provided_by_caregiver,
            date_of_visit,
            pushed_to_redcap,
            redcap_instance,
        } = self;

        let result: CaseNote = sqlx::query_as(
            "
                INSERT INTO case_notes (
                    participant_id,
                    location,
                    visit_type,
                    age,
                    reason_for_visit,
                    info_provided_by_caregiver,
                    date_of_visit,
                    pushed_to_redcap,
                    redcap_instance
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING *
                ",
        )
        .bind(participant)
        .bind(location)
        .bind(visit_type)
        .bind(age)
        .bind(reason_for_visit)
        .bind(info_provided_by_caregiver)
        .bind(date_of_visit)
        .bind(pushed_to_redcap)
        .bind(redcap_instance)
        .fetch_one(database)
        .await?;

        Ok(result)
    }
}
impl Default for NewCaseNote {
    fn default() -> Self {
        Self {
            location: Default::default(),
            visit_type: Some(VisitType::Onsite),
            age: Default::default(),
            reason_for_visit: Default::default(),
            info_provided_by_caregiver: Default::default(),
            date_of_visit: Local::now().date_naive(),
            pushed_to_redcap: false,
            redcap_instance: Default::default(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct NewBloodPressure {
    pub systolic: i16,
    pub diastolic: i16,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, ToSchema)]
pub struct NewCaseNoteHealthMeasures {
    pub sit: Option<NewBloodPressure>,
    pub stand: Option<NewBloodPressure>,
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

impl NewCaseNoteHealthMeasures {
    pub async fn insert_return_none(
        self,
        case_note: i32,
        database: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    ) -> DBResult<()> {
        let Self {
            sit,
            stand,
            weight,
            glucose_tested,
            glucose_result,
            fasted_atleast_2_hours,
            other,
        } = self;

        let (sit_sys, sit_dia) =
            sit.map_or((None, None), |bp| (Some(bp.systolic), Some(bp.diastolic)));
        let (stand_sys, stand_dia) =
            stand.map_or((None, None), |bp| (Some(bp.systolic), Some(bp.diastolic)));

        sqlx::query(
            "
                INSERT INTO case_note_health_measures (
                    case_note_id,
                    blood_pressure_sit_systolic,
                    blood_pressure_sit_diastolic,
                    blood_pressure_stand_systolic,
                    blood_pressure_stand_diastolic,
                    weight,
                    glucose_tested,
                    glucose_result,
                    fasted_atleast_2_hours,
                    other
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ",
        )
        .bind(case_note)
        .bind(sit_sys)
        .bind(sit_dia)
        .bind(stand_sys)
        .bind(stand_dia)
        .bind(weight)
        .bind(glucose_tested)
        .bind(glucose_result)
        .bind(fasted_atleast_2_hours)
        .bind(other)
        .execute(database)
        .await?;
        Ok(())
    }
}
