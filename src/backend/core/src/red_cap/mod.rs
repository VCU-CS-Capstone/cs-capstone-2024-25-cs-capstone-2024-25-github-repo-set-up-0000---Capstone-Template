use std::{any::type_name, collections::HashMap, str::FromStr};

use api::utils::{is_check_box_item, CheckboxValue, FieldNameAndIndex};
use chrono::NaiveDate;
use tracing::warn;
pub mod converter;
mod enum_types;
pub use enum_types::*;
pub mod utils;

pub mod api;
pub type RedCapDataMap = HashMap<String, RedCapExportDataType>;
pub trait RedCapEnum {
    /// To Prevent Obscure Bugs. It will return None
    fn from_usize(value: usize) -> Option<Self>
    where
        Self: Sized;

    fn to_usize(&self) -> usize;
}
pub trait MultiSelectType: RedCapEnum {
    fn from_multi_select(multi_select: &MultiSelect) -> Option<Vec<Self>>
    where
        Self: Sized,
    {
        let mut result = Vec::new();

        for (id, value) in multi_select.set_values.iter() {
            if value == &CheckboxValue::Checked {
                if let Some(value) = Self::from_usize(*id) {
                    result.push(value);
                } else {
                    warn!(?id, "Unknown {}", type_name::<Self>());
                }
            }
        }
        Some(result)
    }
}
pub trait RedCapType {
    fn read(data: &impl RedCapDataSet) -> Option<Self>
    where
        Self: Sized;
}
pub trait RedCapDataSet {
    fn get(&self, key: &str) -> Option<&RedCapExportDataType>;

    fn get_number(&self, key: &str) -> Option<usize> {
        self.get(key).and_then(|value| value.to_number())
    }

    fn get_date(&self, key: &str) -> Option<NaiveDate> {
        self.get(key).and_then(|value| value.to_date())
    }

    fn get_enum<T>(&self, key: &str) -> Option<T>
    where
        T: RedCapEnum,
    {
        self.get(key).and_then(|value| value.to_enum())
    }
    fn get_enum_multi_select<T>(&self, key: &str) -> Option<Vec<T>>
    where
        T: MultiSelectType,
    {
        self.get(key).and_then(|value| value.process_multiselect())
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|value| value.to_string())
    }
    fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|value| value.to_bool())
    }
}
impl RedCapDataSet for RedCapDataMap {
    fn get(&self, key: &str) -> Option<&RedCapExportDataType> {
        self.get(key)
    }
}
#[derive(Debug, Clone)]
pub struct MultiSelect {
    pub field_base: String,
    pub set_values: HashMap<usize, CheckboxValue>,
}

pub fn find_and_extract_multi_selects(items: &mut HashMap<String, String>) -> Vec<MultiSelect> {
    let mut multi_selects = HashMap::new();
    let keys = items
        .keys()
        .filter(|key| is_check_box_item(key.as_str()))
        .cloned()
        .collect::<Vec<String>>();
    for key in keys {
        let value = items.remove(&key).unwrap();
        let FieldNameAndIndex { field_name, index } =
            FieldNameAndIndex::from_str(key.as_str()).unwrap();
        let multi_select = multi_selects
            .entry(field_name.clone())
            .or_insert_with(|| MultiSelect {
                field_base: field_name,
                set_values: HashMap::new(),
            });
        let index = index.unwrap();

        let checkbox_value = CheckboxValue::from_str(value.as_str()).unwrap();

        multi_select.set_values.insert(index, checkbox_value);
    }
    multi_selects.into_values().collect()
}
#[derive(Debug, Clone)]
pub enum RedCapExportDataType {
    MultiSelect(MultiSelect),
    Text(String),
    Null,
    Number(usize),
    Date(NaiveDate),
}
impl RedCapExportDataType {
    pub fn process_string(value: String) -> Self {
        if value.is_empty() {
            Self::Null
        } else if let Ok(number) = value.parse::<usize>() {
            Self::Number(number)
        } else if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
            Self::Date(date)
        } else {
            Self::Text(value)
        }
    }
    pub fn to_string(&self) -> Option<String> {
        match self {
            Self::Text(value) => Some(value.clone()),
            Self::Number(value) => Some(value.to_string()),
            Self::Date(value) => Some(value.format("%Y-%m-%d").to_string()),
            _ => None,
        }
    }
    pub fn to_number(&self) -> Option<usize> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }
    pub fn to_date(&self) -> Option<NaiveDate> {
        match self {
            Self::Date(value) => Some(*value),
            _ => None,
        }
    }
    pub fn to_enum<T>(&self) -> Option<T>
    where
        T: RedCapEnum,
    {
        match self {
            Self::Number(value) => T::from_usize(*value),
            _ => None,
        }
    }
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Self::Number(value) => Some(*value == 1),
            _ => None,
        }
    }
    pub fn as_multiselect(&self) -> Option<&MultiSelect> {
        match self {
            Self::MultiSelect(value) => Some(value),
            _ => None,
        }
    }
    pub fn process_multiselect<T: MultiSelectType>(&self) -> Option<Vec<T>> {
        match self {
            Self::MultiSelect(value) => T::from_multi_select(value),
            _ => None,
        }
    }
}

pub fn process_flat_json(
    mut input: HashMap<String, String>,
) -> HashMap<String, RedCapExportDataType> {
    let multi_selects = find_and_extract_multi_selects(&mut input);

    let mut output = HashMap::new();
    for multi_select in multi_selects {
        output.insert(
            multi_select.field_base.clone(),
            RedCapExportDataType::MultiSelect(multi_select),
        );
    }
    for (key, value) in input {
        let value = RedCapExportDataType::process_string(value);
        output.insert(key, value);
    }
    output
}
