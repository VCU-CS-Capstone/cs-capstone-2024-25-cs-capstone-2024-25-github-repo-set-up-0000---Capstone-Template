use chrono::{DateTime, FixedOffset};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

use super::{
    new::{NewQuestion, NewQuestionCategory, NewQuestionOptions},
    DBResult,
};
/// Table name: _default_questions
#[derive(Debug, FromRow)]
pub struct DefaultQuestionsTable {
    pub id: i32,
    pub file_name: String,
    pub added_at: DateTime<FixedOffset>,
}
impl DefaultQuestionsTable {
    pub async fn was_file_added(file_name: &str, conn: &PgPool) -> DBResult<bool> {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM _default_questions WHERE file_name = $1")
                .bind(file_name)
                .fetch_one(conn)
                .await?;
        Ok(count > 0)
    }

    pub async fn insert_file(file_name: &str, conn: &PgPool) -> DBResult<()> {
        sqlx::query("INSERT INTO _default_questions (file_name) VALUES ($1)")
            .bind(file_name)
            .execute(conn)
            .await?;
        Ok(())
    }
}
#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/questions"]
struct DefaultQuestionsData;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultQuestionWithOptions {
    pub question: NewQuestion,
    pub options: Option<Vec<NewQuestionOptions>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultQuestions {
    pub category: NewQuestionCategory,
    #[serde(default)]
    pub questions: Vec<DefaultQuestionWithOptions>,
}
pub async fn add_default_questions(conn: &PgPool) -> DBResult<()> {
    for question_file in DefaultQuestionsData::iter() {
        if question_file == "README.md" {
            continue;
        }
        if DefaultQuestionsTable::was_file_added(&question_file, conn).await? {
            continue;
        }
        let question = DefaultQuestionsData::get(&question_file).expect("File Should Exist");
        let question: DefaultQuestions =
            serde_json::from_slice(&question.data).expect("This is a bug in the code");
        let DefaultQuestions {
            category,
            questions,
        } = question;
        let category = category.insert_return_category(conn).await?;
        for question in questions {
            let DefaultQuestionWithOptions { question, options } = question;
            let question = question
                .insert_with_category_return_question(category.id, conn)
                .await?;
            if let Some(options) = options {
                for option in options {
                    option
                        .insert_with_question_return_options(question.id, conn)
                        .await?;
                }
            }
        }
        DefaultQuestionsTable::insert_file(&question_file, conn).await?;
    }

    Ok(())
}
