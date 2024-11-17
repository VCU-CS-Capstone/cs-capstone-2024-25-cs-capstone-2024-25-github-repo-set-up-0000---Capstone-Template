//! Due to the amount of data that can be put into red cap. Questions that do not need to be answers at all times or have conditional requirements are stored using a question system.
use crate::database::prelude::*;
use cs25_303_macros::Columns;
use serde::{Deserialize, Serialize};
pub mod default;
pub mod new;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum QuestionType {
    MultiCheckBox,
    Radio,
    Text,
    Number,
    Boolean,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]

pub struct QuestionCategory {
    pub id: i32,
    pub string_id: String,
    pub name: String,
    pub description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]

pub struct Question {
    pub id: i32,
    pub category_id: i32,
    pub question_type: QuestionType,
    pub question: String,
    pub red_cap_id: String,
    pub red_cap_other_id: Option<String>,
    pub removed: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]
pub struct QuestionOptions {
    pub id: i32,
    pub question_id: i32,
    pub name: String,
    pub description: Option<String>,
    /// Will make converting from RedCap easier
    pub red_cap_option_index: Option<i32>,
}
/// Currently many requirements are treated as OR
///
/// This needs to be updated to be more flexible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]
pub struct QuestionRequirements {
    pub id: i32,
    pub question_to_check: i32,
    pub question_to_add: i32,

    pub has_option: Option<i32>,
    pub equals_text: Option<String>,
    pub equals_number: Option<i32>,
    pub equals_radio: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]
pub struct CaseNoteQuestionAnswers {
    pub id: i32,
    pub case_note_id: i32,
    pub question_id: i32,
    pub value_text: Option<String>,
    pub value_number: Option<i32>,
    pub value_radio: Option<i32>,
    pub value_boolean: Option<bool>,
}
/// Table Name: question_answer_multi_check_box
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]
pub struct QuestionAnswerMultiCheck {
    pub id: i32,
    pub question_answers_id: i32,
    pub option_id: i32,
}
