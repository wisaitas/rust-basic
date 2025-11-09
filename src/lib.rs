pub mod config;
pub mod domain;
pub mod middleware;
pub mod usecase;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    let database_url = config::CONFIG.postgres.database_url();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

// Rocket ใช้ Responder trait แทน HttpResponse
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HttpResponse<T> {
    pub status: u16,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error: String,
}

impl<T: Serialize> HttpResponse<T> {
    pub fn success(data: T) -> Self {
        HttpResponse {
            status: 200,
            data: Some(data),
            message: None,
        }
    }

    pub fn created(data: T) -> Self {
        HttpResponse {
            status: 201,
            data: Some(data),
            message: None,
        }
    }
}

pub fn error_response(status: u16, message: String) -> HttpResponse<()> {
    HttpResponse {
        status,
        data: None,
        message: Some(message),
    }
}

// Helper function สำหรับ log errors พร้อม location
#[macro_export]
macro_rules! log_error {
    ($status:expr, $message:expr, $error:expr) => {{
        let location = std::panic::Location::caller();
        eprintln!(
            "[ERROR] {} | Status: {} | File: {}:{}:{} | Message: {} | Error: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC"),
            $status,
            location.file(),
            location.line(),
            location.column(),
            $message,
            $error
        );
    }};
}
