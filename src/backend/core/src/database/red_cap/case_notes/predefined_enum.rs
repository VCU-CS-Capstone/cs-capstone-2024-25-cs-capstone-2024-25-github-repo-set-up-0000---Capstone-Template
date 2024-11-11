use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

/// Red Cap ID: health_ed
///
/// ## Values
///- Diabetes
///- Hypertension
///- Heart failure
///- Mental health
///- Medications
///- Mobility (wheelchair/walker safety)
///- Pain
///- Memory
///- Other
///- N/A - no disease/medication related education
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct DiseaseAndMedicationEducation {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

/// Red Cap ID: health_beh
/// ## Values
///- Diabetes management
///- Hypertension management
///- Managing heart failure
///- Medication adherence
///- Weight
///- Diet
///- Smoking reduction
///- Alcohol use
///- Pain Management
///- Use of blood pressure cuff
///- Goal setting
///- Mental Health
///- Other
///- N/A - no health behavior education
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct HealthBehaviorEducation {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

/// Red Cap ID: visit_collab
/// ## Values
/// - PCP
/// - CAHN Van
/// - Pharmacy
/// - Specialist
/// - OT/PT/rehab
/// - Psych/Mental Health
/// - Home Health
/// - Dental
/// - Health Insurance
/// - Other
/// - N/A - no coordination this visit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct CoordinatedCareWith {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
/// Red Cap ID: visit_referral
/// ## Values
/// - PCP
/// - CAHN Van
/// - Specialist
/// - OT/PT/Rehab
/// - Psych/Mental Health
/// - Home Health
/// - Community Agency
/// - DME
/// - Adult Protective Services
/// - Dental
/// - Glasses/New Eyes for the Needy
/// - Opthamology
/// - Other
/// - N/A - no resources this visit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ResourcesToFor {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
/// Red Cap ID: visit_sdh_edu
///
/// ## Values
/// -Health Insurance
/// -Transportation
/// -Housing
/// -Food/Nutrition
/// -Telephone service
/// -Health Literacy
/// -Personal Technology
/// -Other
/// -N/A - not discussed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct EducationRelatedTo {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
/// Red Cap ID: visit_sdh_referral
/// ## Values
///- Financial (e.g. SSI)
///- Health Insurance (Medicaid, Medicare, etc)
///- Transportation
///- Housing
///- Food/Nutrition
///- Telephone Service
///- Other
///- N/A - no coordination or resources provided
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]

pub struct ServiceCoordination {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
/// Red Cap ID: visit_faculty
///
/// ## Values
///- BSN
///- APRN
///- PharmD
///- OT
///- PT
///- CHW
///- Case Manager
///- MD
///- SW
///- Psychology
///- Other
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct StaffType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

///
/// ## Values
///- PharmD
///- MD
///- BSN
///- AD Nursing
///- BSW/MSW
///- OT
///- NP
///- Psychology
///- Health Science
///- PT
///- Other
///- NO STUDENTS
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct StudentType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
/// Red Cap ID: visit_screens
///
/// # Values
/// - N/A - NO ASSESSMENTS COMPLETED THIS VISIT
/// -  FRAIL questionnaire
/// -  MiniCog
/// -  Social Determinants of Health Screen
/// -  USDA Food Insecurity
/// -  Vulnerable Elders Survey (VES-13)
/// -  General Anxiety Screen (GAD-7)
/// -  Patient Health Questionnaire (PHQ-4)
/// -  Patient Health Questionnaire (PHQ-9)
/// -  Social Connectedness Screen
/// -  Housing Insecurity Screen
/// -  TAPS 1/2
/// -  PROMIS Global Health
/// -  Chronic Disease Self Efficacy Scale
/// -  Other
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ScreeningsCompleted {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

/// Red Cap ID: required_screen
/// # Values
/// - SDOH
/// - PHQ4
/// - PROMIS Global Health
/// - Chronic Disease Self-Efficacy
/// - TAPS 1/2
/// - Other screenings(s)
/// - No screening completed
/// - Data Upload March 2023
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct FacultyNoteScreeningTools {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
/// Red Cap ID: pilot_gaps_coordination
/// # Values
///- Clinic staff / time limitation
///- Participant not interested in resource or open to care coordination
///- Other visit concerns took priority
///- Participant tired / left
///- Multiple follow-up plan elements addressed but not all
///- ALL follow-up plans addressed this visit!
pub struct PDSAToAddressGaps {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
