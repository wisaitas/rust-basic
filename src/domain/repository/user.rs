use crate::domain::entity::user::{User, tbl_users};
use diesel::prelude::*;
use std::error::Error;

pub trait UserRepository {
    fn create(&mut self, user: User) -> Result<User, Box<dyn Error>>;
    fn find_by_email(&mut self, email: &str) -> Result<Option<User>, Box<dyn Error>>;
}

pub struct UserRepositoryImpl {
    postgres: PgConnection,
}

impl UserRepository for UserRepositoryImpl {
    fn create(&mut self, user: User) -> Result<User, Box<dyn Error>> {
        let user = diesel::insert_into(tbl_users::table)
            .values(user)
            .get_result::<User>(&mut self.postgres)?;

        Ok(user)
    }

    fn find_by_email(&mut self, email_param: &str) -> Result<Option<User>, Box<dyn Error>> {
        use crate::domain::entity::user::tbl_users::dsl::*;

        let user = tbl_users
            .filter(email.eq(email_param))
            .filter(deleted_at.is_null())
            .first::<User>(&mut self.postgres)
            .optional()?;

        Ok(user)
    }
}
