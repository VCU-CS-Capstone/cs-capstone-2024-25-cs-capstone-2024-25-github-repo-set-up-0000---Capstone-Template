use cs25_303_macros::EnumWithOther;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum Programs {
    /// Richmond Health And Wellness Program
    RHWP,
    /// Mobile Health And Wellness Program
    MHWP,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum Status {
    Active,
    Inactive,
    NoValidContactStatus,
    Deceases,
    Withdrew,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum SeenAtVCUHS {
    Yes,
    No,
    Unsure,
    DidNotAsk,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumWithOther)]

pub enum Gender {
    #[my_attr("female")]
    Female,
    #[my_attr("male")]
    Male,
    #[my_attr("transgender")]
    Transgender,
    #[my_attr("NonBinary")]
    NonBinary,

    #[my_attr("PreferNotToAnswer")]
    PreferNotToAnswer,
    #[my_attr(other)]
    PreferToSelfDescribe(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum Race {
    NativeAmerican,
    Asian,
    Black,
    Hispanic,
    MiddleEasternOrNorthAfrican,
    NativeHawaiianOrOtherPacificIslander,
    White,
    /// Will have a second field with a value in DB
    Multiracial,
    /// Will have a second field with a value in DB
    IdentifyOther,
    Declined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum Ethnicity {
    HispanicOrLatino,
    NotHispanicOrLatino,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumWithOther)]
pub enum PreferredLanguage {
    #[my_attr("en-US")]
    EnUs,
    #[my_attr("es")]
    Spanish,
    #[my_attr("Asl")]
    Asl,
    #[my_attr(other)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum HealthInsurance {
    Medicaid,
    Medicare,
    Private,
    VA,
    None,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum DegreeLevel {
    None,
    Nursery,
    SomeHighSchool,
    HighschoolOrGED,
    SomeCollege,
    Trade,
    Associates,
    Bachelors,
    Graduates,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum MobilityDevice {
    None,
    Cane,
    Walker,
    Rollator,
    ManualWheelchair,
    PowerWheelchair,
    PowerScooter,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumWithOther)]

pub enum MedicationFrequency {
    #[my_attr("Daily")]
    Daily,
    #[my_attr("TwiceADay")]
    TwiceADay,
    #[my_attr("ThriceADay")]
    ThriceADay,
    #[my_attr("FourTimesADay")]
    FourTimesADay,
    #[my_attr("AsNeeded")]
    AsNeeded,
    #[my_attr(other)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum VisitType {
    Onsite,
    HomeVisit,
    OnsiteAndHome,
    Telephone,
    RBHIAndRHWP,
    PPPAndRHWP,
}
