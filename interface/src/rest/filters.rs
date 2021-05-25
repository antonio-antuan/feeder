use crate::db::models::User;
use crate::db::Pool;
use warp::Filter;

pub fn with_db(
    db: Pool,
) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn with_user(user: User) -> impl Filter<Extract = (User,), Error = std::convert::Infallible> {
    warp::any().map(move || user.clone())
}
