use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteMedications {
    pub id: i64,
    /// 1:1 with [super::CaseNote]
    pub case_note_id: i64,
    /// Red Cap ID: changemeds
    pub has_medications_changed_since_last_visit: Option<bool>,
    /// Red Cap ID: opiod
    pub new_medication_is_opioid: Option<bool>,
    /// Red Cap ID med_rec
    pub confirmed_medication_list: Option<bool>,
    /// Red Cap ID: med_list_discrep
    pub has_discrepancies: Option<bool>,
    /// Red Cap ID: med_notes
    pub disrepancies: Option<String>,
    /// Might be an Enum
    ///
    /// Red Cap ID: adherence
    pub medication_adherence: Option<String>,
    /// Red Cap ID: med_ed
    pub did_provide_medication_education: Option<String>,
    /// Notes: medication_other
    pub provided_medication_education: Option<String>,
    /// Red Cap ID: med_list
    pub provided_medication_list: Option<bool>,
    /// Red Cap ID: med_adher
    pub did_provide_medication_adherence_assistance: Option<bool>,
    /// Red Cap ID: adher_assist_notes
    pub provided_medication_adherence_assistance: Option<String>,
    /// Red Cap ID: contact_pharm
    pub was_pharmacy_contacted: Option<bool>,
    /// Red Cap ID: pharm_call
    pub pharmacy_contacted: Option<String>,
    /// Red Cap ID: med_pc
    pub was_pcp_contacted: Option<bool>,
    /// Red Cap ID: pcp_call
    pub pcp_contacted: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteOpioidScreening {
    pub id: i64,
    /// 1:(1 or 0) relationship with [super::CaseNote]
    pub case_note_id: i64,
    pub prescribed_opioids: bool,
    pub use_of_non_prescribed_opioids: bool,
    // TODO Rest of the fields
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CaseNoteFallScreening {
    pub id: i64,
    /// 1:(1 or 0) relationship with [super::CaseNote]
    pub case_note_id: i64,
    /// RedCap: falls1
    pub have_fallen_since_last_visit: bool,
    /// RedCap: falls2
    pub had_fall_in_last_6_months: bool,
    /// RedCap: falls3
    pub do_you_feel_unstead_when_you_walk_stand: bool,
    /// RedCap: falls4
    pub feel_dizzy_when_sitting_or_laying_down: bool,
    /// RedCap: falls5
    pub are_worried_about_falling: bool,
    /// Redcap: screen_falls1a
    pub circumstances_of_fall: Option<String>,
    pub did_fall_result_in_injury: bool,
    // TODO: Add the rest of the possibilities
}
/// Red Cap ID: visit_screens
///
/// Add Contrainst That Either `screening_id` or `custom_screening_name` is not null
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct GenericScreening {
    pub id: i64,
    /// 1:many relationship with [super::CaseNote]
    pub case_note_id: i64,
    /// Red Cap ID: visit_screens
    pub screening_id: Option<i32>,
    /// Red Cap ID: other_screen
    pub custom_screening_name: Option<String>,
    /// Not in Red Cap. But I think it might be useful
    pub screening_notes: Option<String>,
}
