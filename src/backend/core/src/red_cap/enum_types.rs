use crate::utils::InvalidVariant;
use cs25_303_macros::RedCapEnum;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

use crate::red_cap::{utils::is_all_none, MultiSelectType, RedCapDataSet, RedCapEnum, RedCapType};
/// Returns none if all the fields are none

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, RedCapEnum)]
pub enum Programs {
    /// Richmond Health And Wellness Program
    #[default]
    #[red_cap(enum_index = 1)]
    RHWP,
    /// Mobile Health And Wellness Program
    #[red_cap(enum_index = 2)]
    MHWP,
}

#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum Status {
    #[red_cap(enum_index = 1)]
    Active,
    #[red_cap(enum_index = 0)]
    Inactive,
    #[red_cap(enum_index = 3)]
    NoValidContactStatus,
    #[red_cap(enum_index = 4)]
    Deceases,
    #[red_cap(enum_index = 5)]
    Withdrew,
}

#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum SeenAtVCUHS {
    #[red_cap(enum_index = 1)]
    Yes,
    #[red_cap(enum_index = 0)]
    No,
    #[red_cap(enum_index = 2)]
    Unsure,
    #[red_cap(enum_index = 3)]
    DidNotAsk,
}

#[derive(Debug, Clone, Serialize)]
pub struct RedCapGender {
    pub gender: Option<Gender>,
    pub gender_self: Option<String>,
}
impl RedCapType for RedCapGender {
    fn read(data: &impl crate::red_cap::RedCapDataSet) -> Option<Self>
    where
        Self: Sized,
    {
        let gender = data.get_enum("gender");
        let gender_self = data.get_string("gender_self");
        is_all_none!(gender, gender_self);
        Some(Self {
            gender,
            gender_self,
        })
    }
}
impl From<Gender> for RedCapGender {
    fn from(value: Gender) -> Self {
        let gender_self = match &value {
            Gender::PreferToSelfDescribe(value) => Some(value.clone()),
            _ => None,
        };
        Self {
            gender: Some(value),
            gender_self,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum Gender {
    #[red_cap(name = "female", enum_index = 2)]
    Female,
    #[red_cap(name = "male", enum_index = 1)]
    Male,
    #[red_cap(enum_index = 3)]
    Transgender,
    #[red_cap(enum_index = 4)]
    NonBinary,
    #[red_cap(enum_index = 6)]
    PreferNotToAnswer,
    #[red_cap(other, enum_index = 5)]
    PreferToSelfDescribe(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct RedCapRace {
    pub race: Option<Vec<Race>>,
    pub race_other: Option<String>,
    pub race_multiracial_other: Option<String>,
}
impl RedCapType for RedCapRace {
    fn read(data: &impl RedCapDataSet) -> Option<Self>
    where
        Self: Sized,
    {
        let race = data.get_enum_multi_select("race");
        let race_other = data.get_string("race_other");
        let race_multiracial_other = data.get_string("race_multiracial_other");
        is_all_none!(race, race_other, race_multiracial_other);
        Some(Self {
            race,
            race_other,
            race_multiracial_other,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum Race {
    #[red_cap(enum_index = 3)]
    NativeAmerican,
    #[red_cap(enum_index = 4)]
    Asian,
    #[red_cap(enum_index = 2)]
    Black,
    #[red_cap(enum_index = 5)]
    Hispanic,
    #[red_cap(enum_index = 10)]
    MiddleEasternOrNorthAfrican,
    #[red_cap(enum_index = 7)]
    NativeHawaiianOrOtherPacificIslander,
    #[red_cap(enum_index = 1)]
    White,
    /// Will have a second field with a value in DB
    #[red_cap(enum_index = 9)]
    Multiracial,
    /// Will have a second field with a value in DB
    #[red_cap(enum_index = 6)]
    IdentifyOther,
    #[red_cap(enum_index = 8)]
    Declined,
}
impl MultiSelectType for Race {}
#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum Ethnicity {
    #[red_cap(enum_index = 1)]
    HispanicOrLatino,
    #[red_cap(enum_index = 0)]
    NotHispanicOrLatino,
}

#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum PreferredLanguage {
    #[red_cap(enum_index = 1)]
    EnUs,
    #[red_cap(enum_index = 2)]
    Spanish,
    #[red_cap(enum_index = 3)]
    Asl,
    #[red_cap(other, enum_index = 4)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum HealthInsurance {
    #[red_cap(enum_index = 1)]
    Medicaid,
    #[red_cap(enum_index = 2)]
    Medicare,
    #[red_cap(enum_index = 3)]
    Private,
    #[red_cap(enum_index = 4)]
    VA,
    #[red_cap(enum_index = 5)]
    None,
}
impl MultiSelectType for HealthInsurance {}
#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum DegreeLevel {
    #[red_cap(enum_index = 1)]
    None,
    #[red_cap(enum_index = 2)]
    Nursery,
    #[red_cap(enum_index = 3)]
    SomeHighSchool,
    #[red_cap(enum_index = 4)]
    HighschoolOrGED,
    #[red_cap(enum_index = 5)]
    SomeCollege,
    #[red_cap(enum_index = 6)]
    Trade,
    #[red_cap(enum_index = 7)]
    Associates,
    #[red_cap(enum_index = 8)]
    Bachelors,
    #[red_cap(enum_index = 9)]
    Graduates,
}

#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum MobilityDevice {
    #[red_cap(enum_index = 1)]
    None,
    #[red_cap(enum_index = 2)]
    Cane,
    #[red_cap(enum_index = 3)]
    Walker,
    #[red_cap(enum_index = 4)]
    Rollator,
    #[red_cap(enum_index = 5)]
    ManualWheelchair,
    #[red_cap(enum_index = 6)]
    PowerWheelchair,
    #[red_cap(enum_index = 7)]
    PowerScooter,
}
impl MultiSelectType for MobilityDevice {}

#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum MedicationFrequency {
    #[red_cap(name = "Daily", enum_index = 1)]
    Daily,
    #[red_cap(name = "TwiceADay", enum_index = 2)]
    TwiceADay,
    #[red_cap(enum_index = 3)]
    ThriceADay,
    #[red_cap(enum_index = 4)]
    FourTimesADay,
    #[red_cap(enum_index = 5)]
    AsNeeded,
    #[red_cap(other, enum_index = 6)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedCapMedicationFrequency {
    pub frequency: Option<MedicationFrequency>,
    pub frequency_other: Option<String>,
}
impl RedCapMedicationFrequency {
    pub fn read_with_index(data: &impl RedCapDataSet, index: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let frequency = data.get_enum(format!("frequency{}", index).as_str());
        let frequency_other = data.get_string(format!("other_med{}", index).as_str());
        is_all_none!(frequency, frequency_other);
        Some(Self {
            frequency,
            frequency_other,
        })
    }
}
impl From<RedCapMedicationFrequency> for MedicationFrequency {
    fn from(value: RedCapMedicationFrequency) -> Self {
        match value.frequency {
            Some(MedicationFrequency::Other(value)) => MedicationFrequency::Other(value),
            Some(value) => value,
            None => panic!("Frequency should not be none"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, RedCapEnum)]
pub enum MedStatus {
    #[red_cap(enum_index = 1)]
    Current,
    #[red_cap(enum_index = 2)]
    Discontinued,
}
impl From<MedStatus> for bool {
    fn from(value: MedStatus) -> Self {
        match value {
            MedStatus::Current => true,
            MedStatus::Discontinued => false,
        }
    }
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
