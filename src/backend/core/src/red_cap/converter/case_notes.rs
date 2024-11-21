use ahash::{HashMap, HashMapExt};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

use crate::{
    database::red_cap::{
        case_notes::{
            new::{NewBloodPressure, NewCaseNote, NewCaseNoteHealthMeasures},
            CaseNote, CaseNoteHealthMeasures,
        },
        locations::RedCapLocationConnectionRules,
        questions::{Question, QuestionDataValue, QuestionOptions, QuestionType},
    },
    red_cap::{MultiSelect, RedCapDataSet, RedCapExportDataType, RedCapType, VisitType},
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
    pub red_cap_instance: Option<i32>,
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
            red_cap_instance: redcap_instance,
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
            last_synced_with_redcap: Some(Local::now().fixed_offset()),
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
            red_cap_instance,
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
            red_cap_instance,
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
            red_cap_instance,
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
        if let Some(redcap_instance) = red_cap_instance {
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
            red_cap_instance: Some(redcap_repeat_instance as i32),
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
    ) -> Result<Self, RedCapConverterError> {
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

        Ok(result)
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherCaseNoteData {
    pub values: HashMap<i32, QuestionDataValue>,
}

impl OtherCaseNoteData {
    pub async fn read(
        data: &HashMap<String, RedCapExportDataType>,
        converter: &mut RedCapConverter,
    ) -> Result<Self, RedCapConverterError> {
        let mut values: HashMap<i32, QuestionDataValue> = HashMap::new();
        for (key, value) in data {
            let question =
                Question::find_by_string_id_or_other(key, &key, &converter.database).await?;
            if let Some(question) = question {
                let question_data = match value {
                    RedCapExportDataType::MultiSelect(multi_select) => {
                        if !question.question_type.is_multi_check_box() {
                            error!(?question, ?key, ?value, "Question is not a multi check box");
                            continue;
                        }
                        let result =
                            process_multiselect(&question, multi_select, &mut values, converter)
                                .await?;
                        if let Some(result) = result {
                            result
                        } else {
                            continue;
                        }
                    }
                    RedCapExportDataType::Text(value) => {
                        if let Some(old_question_value) = values.get_mut(&question.id) {
                            debug!("The Multicheck Box has an Other Field Most Likely");
                            old_question_value.push_other_to_other(value.clone());
                            continue;
                        } else {
                            QuestionDataValue::Text(value.clone())
                        }
                    }
                    RedCapExportDataType::Null => continue,
                    RedCapExportDataType::Float(value) => QuestionDataValue::Float(*value),
                    RedCapExportDataType::Number(number) => match question.question_type {
                        QuestionType::Radio => {
                            let Some(option) =
                                QuestionOptions::find_option_with_red_cap_index_and_in_question(
                                    *number as i32,
                                    question.id,
                                    &converter.database,
                                )
                                .await?
                            else {
                                warn!(?question, ?number, "Option Not Found");
                                continue;
                            };
                            QuestionDataValue::Radio {
                                option,
                                other: None,
                            }
                        }
                        QuestionType::Number => QuestionDataValue::Number(*number as i32),
                        QuestionType::Float => QuestionDataValue::Float(*number as f32),
                        QuestionType::Boolean => {
                            let value = *number == 1;
                            QuestionDataValue::Boolean(value)
                        }
                        _ => {
                            warn!(?question, ?number, "Option Not Found");
                            continue;
                        }
                    },
                    RedCapExportDataType::Date(naive_date) => {
                        QuestionDataValue::Text(naive_date.to_string())
                    }
                };
                values.insert(question.id, question_data);
            } else {
                warn!(?key, ?value, "Question Not Found");
            }
        }
        Ok(Self { values })
    }
}

async fn process_multiselect(
    question: &Question,
    multi_select: &MultiSelect,
    values: &mut HashMap<i32, QuestionDataValue>,
    converter: &mut RedCapConverter,
) -> Result<Option<QuestionDataValue>, RedCapConverterError> {
    let mut options = Vec::new();
    for (index, value) in &multi_select.set_values {
        let Some(option) = QuestionOptions::find_option_with_red_cap_index_and_in_question(
            *index,
            question.id,
            &converter.database,
        )
        .await?
        else {
            warn!(?question, ?index, ?value, "Option Not Found");
            continue;
        };

        debug!(?option, ?index, ?value, "Option Found");

        if value.is_checked() {
            options.push(option);
        }
    }
    if let Some(old_question_value) = values.get_mut(&question.id) {
        debug!("The Multicheck Box has an Other Field Most Likely");
        old_question_value.make_multi_check_with_other(options);
        Ok(None)
    } else {
        return Ok(Some(QuestionDataValue::MultiCheckBox {
            options,
            other: None,
        }));
    }
}
