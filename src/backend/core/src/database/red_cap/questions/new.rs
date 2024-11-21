use crate::database::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{
    Question, QuestionCategory, QuestionColumn, QuestionForm, QuestionOptions,
    QuestionOptionsColumn, QuestionType,
};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]

pub struct NewQuestionCategory {
    pub string_id: String,
    pub name: String,
    pub description: Option<String>,
    pub form: QuestionForm,
}
impl NewQuestionCategory {
    pub async fn insert_return_category(self, conn: &PgPool) -> DBResult<QuestionCategory> {
        let Self {
            string_id,
            name,
            description,
            form,
        } = self;

        let category = sqlx::query_as(
            r#"
            INSERT INTO question_categories (string_id, name, description, form)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&string_id)
        .bind(&name)
        .bind(&description)
        .bind(&form)
        .fetch_one(conn)
        .await?;

        Ok(category)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]

pub struct NewQuestion {
    pub category_id: Option<i32>,
    pub question_type: QuestionType,
    #[serde(default)]
    pub required: bool,
    pub question: String,
    pub string_id: String,
    pub string_id_other: Option<String>,
}

impl NewQuestion {
    pub async fn insert_with_category_return_question(
        self,
        category_id: i32,
        conn: &PgPool,
    ) -> DBResult<Question> {
        let Self {
            question_type,
            question,
            string_id,
            string_id_other,
            required,
            ..
        } = self;

        let question = SimpleInsertQueryBuilder::new(Question::table_name())
            .insert(QuestionColumn::CategoryId, category_id)
            .insert(QuestionColumn::QuestionType, question_type)
            .insert(QuestionColumn::Question, question)
            .insert(QuestionColumn::StringId, string_id)
            .insert(QuestionColumn::StringIdOther, string_id_other)
            .insert(QuestionColumn::Required, required)
            .return_all()
            .query_as::<Question>()
            .fetch_one(conn)
            .await?;

        Ok(question)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct NewQuestionOptions {
    pub question_id: Option<i32>,
    pub name: String,
    pub string_id: Option<String>,
    pub description: Option<String>,
    pub red_cap_option_index: Option<i32>,
    #[serde(default)]
    pub unique: bool,
}

impl NewQuestionOptions {
    pub async fn insert_with_question_return_options(
        self,
        question_id: i32,
        conn: &PgPool,
    ) -> DBResult<QuestionOptions> {
        let Self {
            name,
            description,
            red_cap_option_index,
            string_id,
            unique,
            ..
        } = self;

        let option = SimpleInsertQueryBuilder::new(QuestionOptions::table_name())
            .insert(QuestionOptionsColumn::QuestionId, question_id)
            .insert(QuestionOptionsColumn::Name, name)
            .insert(QuestionOptionsColumn::Description, description)
            .insert(
                QuestionOptionsColumn::RedCapOptionIndex,
                red_cap_option_index,
            )
            .insert(QuestionOptionsColumn::StringId, string_id)
            .insert(QuestionOptionsColumn::UniqueOption, unique)
            .return_all()
            .query_as::<QuestionOptions>()
            .fetch_one(conn)
            .await?;
        Ok(option)
    }
}
