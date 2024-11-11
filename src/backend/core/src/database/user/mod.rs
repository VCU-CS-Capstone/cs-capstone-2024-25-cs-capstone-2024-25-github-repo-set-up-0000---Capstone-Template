use auth::UserAndPasswordAuth;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::database::DBResult;
pub mod auth;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize, ToSchema)]
pub struct User {
    /// The ID of the user.
    pub id: i64,
    /// The username of the user.
    pub username: String,
    /// The email of the user.
    pub email: String,
    /// The first name of the user.
    pub first_name: String,
    /// The last name of the user.
    pub last_name: String,
    pub updated_on: chrono::DateTime<chrono::FixedOffset>,
    pub created_on: chrono::DateTime<chrono::FixedOffset>,
}
/// Finds a user by their email or username.
///
/// If user is found it will also return the password authentication data if it exists.
pub async fn find_user_by_email_or_username_with_password_auth(
    email_or_username: impl AsRef<str>,
    db: &sqlx::PgPool,
) -> DBResult<Option<UserAndPasswordAuth>> {
    // TODO use a SQL JOIN
    let Some(user) = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users
        WHERE email = $1 OR username = $1
        "#,
    )
    .bind(email_or_username.as_ref())
    .fetch_optional(db)
    .await?
    else {
        return Ok(None);
    };

    let password_auth = auth::UserPasswordAuthentication::find_by_user_id(user.id, db).await?;

    Ok(Some(UserAndPasswordAuth {
        user,
        password_auth,
    }))
}

pub async fn find_user_by_id(id: i64, db: &sqlx::PgPool) -> DBResult<Option<User>> {
    let user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE id = $1"#)
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(user)
}
