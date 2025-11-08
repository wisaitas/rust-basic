use crate::domain::entity::user::NewUser;
use crate::domain::repository::user::UserRepository;
use crate::usecase::auth::register::dto::{RegisterRequest, RegisterResponse};
use chrono::Utc;
use std::error::Error;

pub struct RegisterService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> RegisterService<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub fn register(&mut self, req: RegisterRequest) -> Result<RegisterResponse, Box<dyn Error>> {
        // Check if email already exists
        if let Some(_) = self.user_repository.find_by_email(&req.email)? {
            return Err("Email already registered".into());
        }

        // Hash password
        let hashed_password = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)?;

        // Create new user
        let new_user = NewUser {
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            email: req.email.clone(),
            password: hashed_password,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let user = self.user_repository.create(new_user)?;

        Ok(RegisterResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            message: "User registered successfully".to_string(),
        })
    }
}
