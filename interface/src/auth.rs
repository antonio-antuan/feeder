use futures::{future, task, FutureExt};
use pbkdf2::{pbkdf2_check, pbkdf2_simple};
use std::rc::Rc;

use crate::db::models::User;
use crate::db::queries::users as users_queries;
use crate::db::Pool;
use crate::result::Result;
use futures::future::{err, ok, LocalBoxFuture};
use futures::task::Poll;
use std::cell::RefCell;

pub fn hash(password: &str) -> String {
    pbkdf2_simple(password, 5000).unwrap()
}

pub async fn login_user(db_pool: &Pool, login: &str, password: &str) -> Result<User> {
    let user = users_queries::get_user_by_login(db_pool, login.to_string()).await?;
    match check_password(password, user.password()) {
        true => Ok(user),
        false => Err(crate::result::Error::Unauthorized(
            "invalid password".to_string(),
        )),
    }
}

fn check_password(password: &str, hashed_password: &str) -> bool {
    match pbkdf2_check(password, hashed_password) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn auth_user(db_pool: &Pool, token: String) -> Result<Option<User>> {
    users_queries::get_user_by_token(db_pool, token).await
}
