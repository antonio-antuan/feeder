pub mod users {
    use std::convert::TryInto;

    tonic::include_proto!("users");

    impl From<crate::db::models::User> for User {
        fn from(user: crate::db::models::User) -> Self {
            User {
                id: user.id,
                last_read_date: user.last_read_date.timestamp().try_into().unwrap(),
                login: user.login,
                token: "".to_string(),
            }
        }
    }
}
