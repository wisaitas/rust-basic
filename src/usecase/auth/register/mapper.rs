use crate::domain::entity::user::User;
use crate::usecase::auth::register::dto::Request;
use chrono::Utc;
use uuid::Uuid;

pub fn map_request_to_entity(req: Request) -> User {
    User {
        id: Uuid::new_v4(),
        first_name: req.first_name,
        last_name: req.last_name,
        email: req.email,
        password: req.password,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    }
}
