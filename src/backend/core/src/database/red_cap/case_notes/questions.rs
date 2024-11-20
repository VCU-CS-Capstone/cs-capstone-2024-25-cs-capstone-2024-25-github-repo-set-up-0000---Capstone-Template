//! Due to the amount of data that can be put into red cap. Questions that do not need to be answers at all times or have conditional requirements are stored using a question system.
use crate::database::prelude::*;
use cs25_303_macros::Columns;
use serde::{Deserialize, Serialize};

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
