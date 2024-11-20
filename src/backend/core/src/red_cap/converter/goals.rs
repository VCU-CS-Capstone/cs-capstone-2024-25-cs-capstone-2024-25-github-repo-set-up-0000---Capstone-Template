use chrono::NaiveDate;

use crate::red_cap::RedCapDataSet;

use super::RedCapConverterError;
// 1-9
#[derive(Debug, Clone, PartialEq)]
pub struct RedCapGoals {
    // Format ltg{number} 1-9
    pub goal: String,
    // Format active_ltg{number} 1-9
    pub is_active: Option<bool>,
    pub red_cap_index: i32,
}
impl RedCapGoals {
    pub fn read_index<D: RedCapDataSet>(data: &D, index: i32) -> Option<Self> {
        let goal = format!("ltg{}", index);
        let goal = data.get_string(&goal)?;
        let is_active = format!("active_ltg{}", index);
        let is_active = data.get_bool(&is_active);
        Some(Self {
            goal,
            is_active,
            red_cap_index: index,
        })
    }
    pub fn read<D: RedCapDataSet>(data: &D) -> Vec<Self> {
        (1..10).filter_map(|x| Self::read_index(data, x)).collect()
    }
}
//  1 -25
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RedCapGoalsSteps {
    //  associated_goal_{number}
    pub associated_goal: Option<i32>,
    // goal{number}
    pub step: String,
    // confidence{number}
    pub confidence_level: Option<i16>,
    // set{number}
    pub date_set: Option<NaiveDate>,
    // date{number}
    pub date_to_be_completed: Option<NaiveDate>,
    // action{number}
    pub action_step: Option<bool>,
    pub red_cap_index: i32,
}
impl RedCapGoalsSteps {
    pub fn read_index<D: RedCapDataSet>(data: &D, index: i32) -> Option<Self> {
        let goal = format!("goal{}", index);
        let confidence = format!("confidence{}", index);
        let set = format!("set{}", index);
        let date = format!("date{}", index);
        let action = format!("action{}", index);

        let step = data.get_string(&goal)?;
        let confidence_level = data.get_number(&confidence).map(|x| x as i16);
        let date_set = data.get_date(&set);
        let date_to_be_completed = data.get_date(&date);
        let action_step = data.get_bool(&action);
        Some(Self {
            associated_goal: Some(index),
            step,
            confidence_level,
            date_set,
            date_to_be_completed,
            action_step,
            red_cap_index: index,
        })
    }
    pub fn read<D: RedCapDataSet>(data: &D) -> Vec<Self> {
        (1..26).filter_map(|x| Self::read_index(data, x)).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RedCapCompleteGoals {
    pub goals: Vec<RedCapGoals>,
    pub steps: Vec<RedCapGoalsSteps>,
}
impl RedCapCompleteGoals {
    pub async fn read<D: RedCapDataSet>(data: &D) -> Result<Self, RedCapConverterError> {
        let goals = RedCapGoals::read(data);
        let steps = RedCapGoalsSteps::read(data);
        Ok(Self { goals, steps })
    }
}
