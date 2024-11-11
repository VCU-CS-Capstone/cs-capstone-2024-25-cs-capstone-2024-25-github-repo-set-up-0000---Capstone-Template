//! Relates to all the Faculty or Student related data in the case notes
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
/// Faculty, Staff, and Student Related Data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteFacultyAndStaff {
    pub id: i64,
    /// 1:1 with [super::CaseNote]
    pub case_note_id: i64,
    /// Refers to [predefined_enum::StaffType]
    /// Red Cap ID: visit_faculty
    pub faculity_type_id: Option<Vec<i32>>,
    /// Red Cap ID: otherfac
    pub custom_faculity_type: Option<String>,
    /// Red Cap ID: visit_faculty_clinic
    pub faculity_with_participants: Option<i32>,
    /// Red  Cap ID: visit_faculty_edu
    pub faculity_with_students: Option<i32>,
    /// Red Cap ID: facultyname
    pub faculity_name: Option<String>,
    /// Red Cap ID: `fac_note`
    pub faculty_note: Option<String>,
    /// Red Cap ID : required_screen
    /// Refers to [super::predefined_enum::FacultyNoteScreeningTools]
    pub screening_tools: Option<Vec<i32>>,
    /// Red Cap ID: `pilot_gaps_coordination`
    /// Refers to [super::predefined_enum::PDSAToAddressGaps]
    pub pdsa_to_address_gaps: Option<Vec<i32>>,
    /// Not Sure what this value is?
    /// However, this value is technically under the student section however, in our case we have the student section use a 1:many relationship.
    /// This data is a 1:1 relationship with the case note.
    /// Redcap ID: `studenttime`
    pub total_student_time: Option<i32>,
}

/// Red Cap ID: students
/// Constraint: Either `student_type_id` or `custom_student_type` is not null
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteStudents {
    pub id: i64,
    /// 1:many relationship with [super::CaseNote]
    pub case_note_id: i64,
    /// Refers to [predefined_enum::StudentType]
    /// Red Cap ID: student_type
    pub student_type_id: Option<i32>,
    /// When `student_type_id` is `None` this field is used
    pub custom_student_type: Option<String>,
    /// Red Cap ID depends on student type.
    ///
    /// Generally it is `num{student_type}`
    ///
    /// Custom Student Type is `numother`
    pub number_of_students: i32,
}
