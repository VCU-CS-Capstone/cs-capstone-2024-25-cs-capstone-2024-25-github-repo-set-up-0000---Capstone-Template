use crate::database::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{Question, QuestionCategory, QuestionForm, QuestionOptions, QuestionType};

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

    pub question: String,
    pub red_cap_id: String,
    pub red_cap_other_id: Option<String>,
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
            red_cap_id,
            red_cap_other_id,
            ..
        } = self;

        let question = sqlx::query_as(
            r#"
            INSERT INTO questions (category_id, question_type, question, red_cap_id, red_cap_other_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(category_id)
        .bind(question_type)
        .bind(question)
        .bind(red_cap_id)
        .bind(red_cap_other_id)
        .fetch_one(conn)
        .await?;

        Ok(question)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct NewQuestionOptions {
    pub question_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub red_cap_option_index: Option<i32>,
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
            ..
        } = self;

        let options = sqlx::query_as(
            r#"
            INSERT INTO question_options (question_id, name, description, red_cap_option_index)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(question_id)
        .bind(name)
        .bind(description)
        .bind(red_cap_option_index)
        .fetch_one(conn)
        .await?;

        Ok(options)
    }
}
