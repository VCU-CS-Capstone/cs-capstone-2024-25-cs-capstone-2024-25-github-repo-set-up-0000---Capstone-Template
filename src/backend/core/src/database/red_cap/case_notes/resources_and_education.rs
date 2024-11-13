use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteGenericEducation {
    pub id: i32,
    pub case_note_id: i32,
    pub generic_education: Option<i32>,
    pub generic_education_other: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteServiceCoordinated {
    pub id: i32,
    pub case_note_id: i32,
    pub service_coordinated: Option<i32>,
    pub service_coordinated_other: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteDiseaseAndMedicationEducation {
    pub id: i32,
    pub case_note_id: i32,
    pub disease_and_medication_education: Option<i32>,
    pub disease_and_medication_education_other: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteHealthBehaviorEducation {
    pub id: i32,
    pub case_note_id: i32,
    pub health_behavior_education: Option<i32>,
    pub health_behavior_education_other: Option<String>,
}
