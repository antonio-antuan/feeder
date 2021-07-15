use rand;

use crate::db::models::User;
use crate::db::Pool;
use crate::result::Result;

use rand::distributions::Alphanumeric;
use rand::Rng;

pub async fn get_user_by_token(db_pool: &Pool, token: String) -> Result<Option<User>> {
    Ok(
        sqlx::query_as!(User, "SELECT * from users WHERE token = $1", token)
            .fetch_optional(db_pool)
            .await?,
    )
}

pub fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

pub async fn create_user(
    db_pool: &Pool,
    login: String,
    hashed_password: String,
) -> Result<Option<User>> {
    match sqlx::query_as!(
        User,
        r#"INSERT INTO users (login, token, password, last_read_date) VALUES 
        ($1, $2, $3, now()) RETURNING *"#,
        login,
        generate_token(),
        hashed_password,
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            if let sqlx::Error::Database(db_err) = &err {
                if let Some(constraint) = db_err.constraint() {
                    if constraint == "users_login_key" {
                        return Ok(None);
                    }
                }
            }
            Err(err)?
        }
    }
}

pub async fn get_user_by_login(db_pool: &Pool, login: String) -> Result<User> {
    Ok(
        sqlx::query_as!(User, "SELECT * FROM users WHERE login = $1", login)
            .fetch_one(db_pool)
            .await?,
    )
}
