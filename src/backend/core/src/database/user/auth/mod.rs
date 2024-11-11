use sqlx::{prelude::FromRow, PgPool};

use crate::database::{DBResult, DBTime};

use super::User;
#[derive(Debug)]
pub struct UserAndPasswordAuth {
    pub user: User,
    pub password_auth: Option<UserPasswordAuthentication>,
}
/// Table: user_authentication_password
#[derive(Debug, FromRow)]
pub struct UserPasswordAuthentication {
    pub id: i64,
    pub user_id: i64,
    /// Passwords will be stored with Argon2.
    pub password: Option<String>,
    pub password_last_updated: DBTime,
    pub requires_password_reset: bool,
    pub created_on: DBTime,
}
impl UserPasswordAuthentication {
    pub async fn find_by_user_id(user_id: i64, db: &PgPool) -> DBResult<Option<Self>> {
        sqlx::query_as("SELECT * FROM user_authentication_password WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db)
            .await
    }
}
/// Table: user_authentication_saml
///
/// Contains Data for the Users who are authenticated via SAML.
///
/// I don't know how SAML works yet. This is just a template
#[derive(Debug)]
pub struct SamlAuthentication {
    pub id: i64,
    pub user_id: i64,

    pub saml_id: String,
    pub created_on: DBTime,
}
