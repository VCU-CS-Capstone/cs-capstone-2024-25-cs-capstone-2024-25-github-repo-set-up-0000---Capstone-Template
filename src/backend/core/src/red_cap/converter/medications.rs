use chrono::NaiveDate;

use crate::{
    database::red_cap::participants::NewMedication,
    red_cap::{MedStatus, RedCapDataSet, RedCapMedicationFrequency},
};

#[derive(Debug, Clone, PartialEq)]
pub struct RedCapMedication {
    pub name: String,
    pub dosage: Option<String>,
    pub frequency: Option<RedCapMedicationFrequency>,
    pub date_prescribed: Option<NaiveDate>,
    pub date_entered_into_system: Option<NaiveDate>,
    pub status: Option<MedStatus>,
    pub date_discontinued: Option<NaiveDate>,
    pub comments: Option<String>,
    pub red_cap_index: Option<i32>,
}
impl From<RedCapMedication> for NewMedication {
    fn from(value: RedCapMedication) -> Self {
        let RedCapMedication {
            name,
            dosage,
            frequency,
            date_prescribed,
            date_entered_into_system,
            status,
            date_discontinued,
            comments,
            red_cap_index,
        } = value;

        Self {
            name,
            dosage,
            frequency: frequency.map(|x| x.into()),
            date_prescribed,
            date_entered_into_system,
            is_current: status.map(|x| x.into()),
            date_discontinued,
            comments,
            red_cap_index,
        }
    }
}
impl RedCapMedication {
    pub fn read_index<D: RedCapDataSet>(data: &D, index: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let name = data.get_string(format!("med{}", index).as_str())?;
        let dosage = data.get_string(format!("dosage{}", index).as_str());
        let frequency = RedCapMedicationFrequency::read_with_index(data, index);
        let date_prescribed = data.get_date(format!("med_date{}", index).as_str());
        let date_entered_into_system = data.get_date(format!("med_red_{}", index).as_str());
        let status = data.get_enum::<MedStatus>(format!("med_status{}", index).as_str());
        let date_discontinued = data.get_date(format!("med_dis{}", index).as_str());
        let comments = data.get_string(format!("med_other{}", index).as_str());

        Some(Self {
            name,
            dosage,
            frequency,
            date_prescribed,
            date_entered_into_system,
            status,
            date_discontinued,
            comments,
            red_cap_index: Some(index as i32),
        })
    }

    pub fn read<D: RedCapDataSet>(data: &D) -> Vec<Self>
    where
        Self: Sized,
    {
        (1..=40).filter_map(|x| Self::read_index(data, x)).collect()
    }
}
