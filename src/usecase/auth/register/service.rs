use crate::domain::entity::user::User;
use crate::domain::repository::user::UserRepository;
use crate::usecase::auth::register::dto::Request;
use bcrypt;
use chrono::Utc;
use std::error::Error;
use uuid::Uuid;

pub trait RegisterService {
    fn register(&mut self, req: Request) -> Result<RegisterResponse, Box<dyn Error>>;
}

pub struct RegisterServiceImpl {
    user_repository: Box<dyn UserRepository>,
}

impl RegisterServiceImpl {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

impl RegisterService for RegisterServiceImpl {
    fn register(&mut self, req: Request) -> Result<RegisterResponse, Box<dyn Error>> {
        // Check if email already exists
        if let Some(_) = self.user_repository.find_by_email(&req.email)? {
            return Err("Email already registered".into());
        }

        // Hash password
        let hashed_password = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)?;

        // Create new user entity
        let new_user = User {
            id: Uuid::new_v4(),
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            email: req.email.clone(),
            password: hashed_password,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
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

// ใช้ type alias สำหรับ backward compatibility
pub type Service = RegisterServiceImpl;
pub type ServiceImpl = RegisterServiceImpl;

// RegisterResponse type
#[derive(Debug)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub message: String,
}

// ต้องเพิ่ม Serialize trait สำหรับ RegisterResponse
use rocket::serde::Serialize;
impl Serialize for RegisterResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        use rocket::serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RegisterResponse", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}
