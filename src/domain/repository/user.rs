use crate::domain::entity::user::{NewUser, User};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use std::error::Error;

pub trait UserRepository {
    fn create(&mut self, new_user: NewUser) -> Result<User, Box<dyn Error>>;
    fn find_by_email(&mut self, email: &str) -> Result<Option<User>, Box<dyn Error>>;
}

pub struct UserRepositoryImpl {
    pub conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl UserRepository for UserRepositoryImpl {
    fn create(&mut self, new_user: NewUser) -> Result<User, Box<dyn Error>> {
        use crate::schema::user::tbl_users;

        let user = diesel::insert_into(tbl_users::table)
            .values(&new_user)
            .get_result::<User>(&mut self.conn)?;

        Ok(user)
    }

    fn find_by_email(&mut self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        use crate::schema::user::tbl_users::dsl::*;

        let user = tbl_users
            .filter(crate::schema::user::tbl_users::email.eq(email))
            .filter(deleted_at.is_null())
            .first::<User>(&mut self.conn)
            .optional()?;

        Ok(user)
    }
}
