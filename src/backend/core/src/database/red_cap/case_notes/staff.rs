//! Relates to all the Faculty or Student related data in the case notes
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Faculty, Staff, and Student Related Data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteFacultyAndStaff {
    pub id: i32,
    /// 1:1 with [super::CaseNote]
    pub case_note_id: i32,
    /// Red Cap ID: visit_faculty_clinic
    pub faculity_with_participants: Option<i32>,
    /// Red  Cap ID: visit_faculty_edu
    pub faculity_with_students: Option<i32>,
    /// Red Cap ID: facultyname
    pub faculity_name: Option<String>,
    /// Red Cap ID: `fac_note`
    pub faculty_note: Option<String>,
    /// Not Sure what this value is?
    /// However, this value is technically under the student section however, in our case we have the student section use a 1:many relationship.
    /// This data is a 1:1 relationship with the case note.
    /// Redcap ID: `studenttime`
    pub total_student_time: Option<i32>,
}
