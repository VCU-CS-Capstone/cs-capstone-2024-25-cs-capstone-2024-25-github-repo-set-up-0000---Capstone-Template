use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::{
    new::{NewQuestion, NewQuestionCategory, NewQuestionOptions},
    DBResult,
};
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
    for question in DefaultQuestionsData::iter() {
        if question == "README.md" {
            continue;
        }
        let question = DefaultQuestionsData::get(&question).unwrap();
        let question: DefaultQuestions = serde_json::from_slice(&question.data).unwrap();
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
    }

    Ok(())
}
// TODO: IDK How the best way to handle this is. Writing pure sql for all this data will be a pain.
pub async fn should_add_default_questions(conn: &PgPool) -> DBResult<bool> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM question_categories")
        .fetch_one(conn)
        .await?;

    Ok(count == 0)
}
