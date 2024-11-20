use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    database::red_cap::{
        case_notes::{
            new::{NewBloodPressure, NewCaseNote, NewCaseNoteHealthMeasures},
            CaseNote, CaseNoteHealthMeasures,
        },
        locations::RedCapLocationConnectionRules,
    },
    red_cap::{RedCapDataSet, RedCapExportDataType, RedCapType, VisitType},
};

use super::{RedCapConverter, RedCapConverterError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedCapCaseNoteBase {
    pub location: Option<i32>,
    pub visit_type: Option<VisitType>,
    pub age: Option<i16>,
    pub reason_for_visit: Option<String>,
    pub info_provided_by_caregiver: Option<String>,
    pub date_of_visit: NaiveDate,
    pub pushed_to_redcap: bool,
    pub redcap_instance: Option<i32>,
    pub completed: bool,
}
impl From<RedCapCaseNoteBase> for NewCaseNote {
    fn from(value: RedCapCaseNoteBase) -> Self {
        let RedCapCaseNoteBase {
            location,
            visit_type,
            age,
            reason_for_visit,
            info_provided_by_caregiver,
            date_of_visit,
            pushed_to_redcap,
            redcap_instance,
            completed,
        } = value;

        NewCaseNote {
            location,
            visit_type,
            age,
            reason_for_visit,
            info_provided_by_caregiver,
            date_of_visit,
            pushed_to_redcap,
            redcap_instance,
            completed,
        }
    }
}
impl From<CaseNote> for RedCapCaseNoteBase {
    fn from(value: CaseNote) -> Self {
        let CaseNote {
            location,
            visit_type,
            age,
            reason_for_visit,
            info_provided_by_caregiver,
            date_of_visit,
            pushed_to_redcap,
            redcap_instance,
            completed,
            ..
        } = value;

        Self {
            location,
            visit_type,
            age,
            reason_for_visit,
            info_provided_by_caregiver,
            date_of_visit,
            pushed_to_redcap,
            redcap_instance,
            completed,
        }
    }
}
impl RedCapCaseNoteBase {
    pub async fn write_case_note<D: RedCapDataSet>(
        &self,
        data: &mut D,
        converter: &mut RedCapConverter,
    ) -> Result<(), RedCapConverterError> {
        let Self {
            location,
            visit_type,
            age,
            reason_for_visit,
            info_provided_by_caregiver,
            date_of_visit,
            redcap_instance,
            ..
        } = self;

        if let Some(location) = location {
            let location = converter
                .locations
                .iter()
                .find(|x| x.id == *location)
                .expect("Location not found")
                .red_cap_connection_rules
                .visit_rules();
            info!("Location: {:?}", location);
            location.write(data);
        }

        data.insert("visit_type".to_string(), visit_type.clone().into());
        data.insert("exit_age".to_string(), (*age).into());
        data.insert("reason".to_string(), reason_for_visit.clone().into());
        data.insert(
            "subjective_info".to_string(),
            info_provided_by_caregiver.clone().into(),
        );
        data.insert("visit_date".to_string(), (*date_of_visit).into());
        if let Some(redcap_instance) = redcap_instance {
            data.insert(
                "redcap_repeat_instance".to_string(),
                (*redcap_instance).into(),
            );
        } else {
            data.insert(
                "redcap_repeat_instance".to_string(),
                "new".to_owned().into(),
            );
        }
        data.insert(
            "redcap_repeat_instrument".to_string(),
            "case_note".to_string().into(),
        );

        data.insert(
            "case_note_complete".to_string(),
            RedCapExportDataType::from_bad_boolean(self.completed),
        );

        Ok(())
    }
    pub async fn read_case_note_base<D: RedCapDataSet>(
        data: &D,
        converter: &mut RedCapConverter,
    ) -> Result<Option<Self>, RedCapConverterError> {
        // Red Cap Randomly Puts a empty one that doest exist?
        let Some(redcap_repeat_instance) = data.get_number("redcap_repeat_instance") else {
            return Ok(None);
        };
        let location = if let Some(location) = RedCapLocationConnectionRules::read(data) {
            let location = converter.find_location_from_connection_rules_for_visit(&location);
            info!("Location: {:?}", location);
            location.map(|x| x.id)
        } else {
            None
        };

        let result = RedCapCaseNoteBase {
            location,
            visit_type: data.get_enum("visit_type"),
            age: data.get_number("exit_age").map(|x| x as i16),
            reason_for_visit: data.get_string("reason"),
            info_provided_by_caregiver: data.get_string("subjective_info"),
            date_of_visit: data.get_date("visit_date").unwrap_or_default(),
            pushed_to_redcap: true,
            redcap_instance: Some(redcap_repeat_instance as i32),
            completed: data
                .get_bad_boolean("case_note_complete")
                .unwrap_or_default(),
        };

        Ok(Some(result))
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedCapHealthMeasures {
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
impl From<RedCapHealthMeasures> for NewCaseNoteHealthMeasures {
    fn from(value: RedCapHealthMeasures) -> Self {
        let RedCapHealthMeasures {
            sit,
            stand,
            weight,
            glucose_tested,
            glucose_result,
            fasted_atleast_2_hours,
            other,
        } = value;

        NewCaseNoteHealthMeasures {
            sit,
            stand,
            weight,
            glucose_tested,
            glucose_result,
            fasted_atleast_2_hours,
            other,
        }
    }
}
impl From<CaseNoteHealthMeasures> for RedCapHealthMeasures {
    fn from(value: CaseNoteHealthMeasures) -> Self {
        let CaseNoteHealthMeasures {
            weight,
            glucose_tested,
            glucose_result,
            fasted_atleast_2_hours,
            other,
            blood_pressure_sit_diastolic,
            blood_pressure_sit_systolic,
            blood_pressure_stand_diastolic,
            blood_pressure_stand_systolic,
            ..
        } = value;

        let sit = if let (Some(systolic), Some(diastolic)) =
            (blood_pressure_sit_systolic, blood_pressure_sit_diastolic)
        {
            Some(NewBloodPressure {
                systolic,
                diastolic,
            })
        } else {
            None
        };

        let stand = if let (Some(systolic), Some(diastolic)) = (
            blood_pressure_stand_systolic,
            blood_pressure_stand_diastolic,
        ) {
            Some(NewBloodPressure {
                systolic,
                diastolic,
            })
        } else {
            None
        };
        Self {
            sit,
            stand,
            weight,
            glucose_tested,
            glucose_result,
            fasted_atleast_2_hours,
            other,
        }
    }
}
impl RedCapHealthMeasures {
    pub fn write_health_measures<D: RedCapDataSet>(&self, data: &mut D) {
        let RedCapHealthMeasures {
            sit,
            stand,
            weight,
            glucose_tested,
            glucose_result,
            fasted_atleast_2_hours,
            other,
        } = self;
        data.insert("bp_sit", sit.is_some().into());
        if let Some(sit) = sit {
            write_blood_pressure(data, "bp_sit", sit);
        }
        data.insert("bp_stand", stand.is_some().into());
        if let Some(stand) = stand {
            write_blood_pressure(data, "bp_stand", stand);
        }
        data.insert("weight_yn", weight.is_some().into());
        if let Some(weight) = weight {
            data.insert("weight".to_string(), (*weight).into());
        }
        data.insert("glucose_yn".to_string(), (*glucose_tested).into());
        if let Some(glucose_result) = glucose_result {
            data.insert("glucose".to_string(), (*glucose_result).into());
            let glucose_fasting = if *fasted_atleast_2_hours {
                "1".to_owned()
            } else {
                "2".to_owned()
            };
            data.insert("glucose_fasting".to_string(), glucose_fasting.into());
            if let Some(other) = other {
                data.insert("visit_function".to_string(), other.clone().into());
            }
        }
    }
    pub async fn read_health_measures<D: RedCapDataSet>(
        data: &D,
    ) -> Result<Option<Self>, RedCapConverterError> {
        let sit = read_blood_pressure(data, "bp_sit");
        let stand = read_blood_pressure(data, "bp_stand");
        let weight = data.get_number("weight");
        let glucose_tested = data.get_bool("glucose_yn");
        let glucose_result = data.get_number("glucose");
        let fasted_atleast_2_hours = data.get_bool("glucose_fasting");
        let other = data.get_string("visit_function");

        let result = RedCapHealthMeasures {
            sit,
            stand,
            weight: weight.map(|x| x as f32),
            glucose_tested: glucose_tested.unwrap_or_default(),
            glucose_result: glucose_result.map(|x| x as f32),
            fasted_atleast_2_hours: fasted_atleast_2_hours.unwrap_or_default(),
            other,
        };

        Ok(Some(result))
    }
}

fn read_blood_pressure<D: RedCapDataSet>(data: &D, prefix: &str) -> Option<NewBloodPressure> {
    let systolic = data.get_number(&format!("{}_syst", prefix))?;
    let diastolic = data.get_number(&format!("{}_dia", prefix))?;

    Some(NewBloodPressure {
        systolic: systolic as i16,
        diastolic: diastolic as i16,
    })
}
fn write_blood_pressure<D: RedCapDataSet>(data: &mut D, prefix: &str, value: &NewBloodPressure) {
    data.insert(format!("{}_syst", prefix), value.systolic.into());
    data.insert(format!("{}_dia", prefix), value.diastolic.into());
}
