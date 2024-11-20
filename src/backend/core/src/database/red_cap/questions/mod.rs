//! Due to the amount of data that can be put into red cap.
//! Questions that do not need to be answers at all times or have conditional requirements are stored using a question system.
use crate::database::prelude::*;
use cs25_303_macros::Columns;
use serde::{Deserialize, Serialize};
pub mod default;
pub mod new;
/// Where does the question belong to
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum QuestionForm {
    /// Case Notes
    CaseNotes,
    /// Participant Info
    ParticipantInfo,
}
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
    pub form: QuestionForm,
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
impl Question {
    pub async fn find_by_red_cap_id(red_cap_id: &str, conn: &PgPool) -> DBResult<Option<Self>> {
        let question = SimpleSelectQueryBuilder::new(Self::table_name(), &QuestionColumn::all())
            .where_equals(QuestionColumn::RedCapId, red_cap_id)
            .limit(1)
            .query_as::<Self>()
            .fetch_optional(conn)
            .await?;
        Ok(question)
    }
}
impl TableType for Question {
    type Columns = QuestionColumn;
    fn table_name() -> &'static str
    where
        Self: Sized,
    {
        "questions"
    }
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
    pub equals_radio: Option<i32>,

    pub equals_boolean: Option<bool>,
    pub equals_text: Option<String>,
    pub equals_number: Option<i32>,
}
impl TableType for QuestionRequirements {
    type Columns = QuestionRequirementsColumn;
    fn table_name() -> &'static str
    where
        Self: Sized,
    {
        "question_requirements"
    }
}
