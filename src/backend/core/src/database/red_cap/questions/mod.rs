//! Due to the amount of data that can be put into red cap.
//! Questions that do not need to be answered at all times or have conditional requirements are stored using a question system.
//!
//! This prevents a ton of tables and columns with null values.
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
    Float,
    Boolean,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]

pub struct QuestionCategory {
    pub id: i32,
    pub form: QuestionForm,
    /// The string id of the category
    pub string_id: String,
    /// The name of the category
    pub name: String,
    /// Category description
    pub description: Option<String>,
}
impl QuestionCategory {
    pub async fn delete_all(conn: &PgPool) -> DBResult<()> {
        let _ = sqlx::query("DELETE FROM question_categories")
            .execute(conn)
            .await?;
        Ok(())
    }
}
impl TableType for QuestionCategory {
    type Columns = QuestionCategoryColumn;
    fn table_name() -> &'static str
    where
        Self: Sized,
    {
        "question_categories"
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, Columns)]

pub struct Question {
    pub id: i32,
    /// The category the question belongs to
    pub category_id: i32,
    /// MUST CORRESPOND TO RED CAP ID
    pub string_id: String,
    /// For MultiCheckBox and Radio only
    ///
    /// MUST CORRESPOND TO RED CAP ID
    pub string_id_other: Option<String>,
    /// The type of question
    pub question_type: QuestionType,
    /// The name of the question
    pub question: String,
    pub description: Option<String>,
    /// If the question is required
    /// Will be ignored if requirements are not met
    pub required: bool,
    /// If the question is removed
    pub removed: bool,
}
impl Question {
    pub async fn find_by_red_cap_id(red_cap_id: &str, conn: &PgPool) -> DBResult<Option<Self>> {
        let question = SimpleSelectQueryBuilder::new(Self::table_name(), &QuestionColumn::all())
            .where_equals(QuestionColumn::StringId, red_cap_id)
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
    /// A id that is used to reference the option
    pub string_id: Option<String>,
    /// The index of the option in red cap
    pub red_cap_option_index: Option<i32>,
    /// The name of the option
    pub name: String,
    /// Additional description
    pub description: Option<String>,
    /// If an option is unique it can be only selected and no other options
    pub unique_option: bool,
    /// Was the response removed
    pub removed: bool,
}
impl QuestionOptions {
    pub async fn find_option_with_string_id_and_in_question(
        string_id: &str,
        question_id: i32,
        conn: &PgPool,
    ) -> DBResult<Option<Self>> {
        let option =
            SimpleSelectQueryBuilder::new(Self::table_name(), &QuestionOptionsColumn::all())
                .where_equals_then(QuestionOptionsColumn::StringId, string_id, |builder| {
                    builder.and_equals(QuestionOptionsColumn::QuestionId, question_id);
                })
                .limit(1)
                .query_as::<Self>()
                .fetch_optional(conn)
                .await?;
        Ok(option)
    }
}
impl TableType for QuestionOptions {
    type Columns = QuestionOptionsColumn;
    fn table_name() -> &'static str
    where
        Self: Sized,
    {
        "question_options"
    }
}
/// Currently many requirements are treated as OR
///
/// This needs to be updated to be more flexible.
///
/// Also can't reference regular columns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Columns)]
pub struct QuestionRequirements {
    pub id: i32,
    pub question_to_check: i32,
    pub question_to_add: i32,

    pub has_option: Option<i32>,
    pub equals_radio: Option<i32>,

    pub equals_boolean: Option<bool>,
    pub equals_text: Option<String>,
    pub equals_number: Option<i32>,
    pub equals_float: Option<f32>,
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
