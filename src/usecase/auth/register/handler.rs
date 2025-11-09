use crate::ErrorResponse;
use rocket::{Response, State, http::Status, post, response::Responder, serde::json::Json};
use validator::Validate;

use crate::DbPool;
use crate::domain::repository::user::UserRepositoryImpl;
use crate::usecase::auth::register::dto::Request as RegisterRequest;
use crate::usecase::auth::register::service::{
    RegisterResponse, RegisterService, RegisterServiceImpl,
}; // เพิ่ม RegisterResponse

// Custom Responder สำหรับ error handling
pub struct ApiResponse<T>(Status, Json<T>);

impl<'r, T: rocket::serde::Serialize> Responder<'r, 'static> for ApiResponse<T> {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Response::build_from(self.1.respond_to(req)?)
            .status(self.0)
            .ok()
    }
}

#[post("/auth/register", data = "<payload>")]
pub async fn register_handler(
    pool: &State<DbPool>,
    payload: Json<RegisterRequest>,
) -> Result<ApiResponse<RegisterResponse>, ApiResponse<ErrorResponse>> {
    // Validate request
    if let Err(errors) = payload.validate() {
        let error_message = errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                format!(
                    "{}: {}",
                    field,
                    errs[0].message.as_ref().unwrap_or(&"Invalid".into())
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        // Log error (แต่ไม่ส่ง details ไปให้ client)
        crate::log_error!(400, "Validation error", &error_message);

        // ส่ง generic message ให้ client
        return Err(ApiResponse(
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Invalid request data".to_string(),
            }),
        ));
    }

    // Get connection from pool
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            // Log detailed error
            crate::log_error!(500, "Database connection error", &format!("{}", e));

            // ส่ง generic message ให้ client
            return Err(ApiResponse(
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            ));
        }
    };

    // Create repository and service
    let user_repo = UserRepositoryImpl { conn };
    let mut service = RegisterServiceImpl::new(Box::new(user_repo));

    // Process registration
    match service.register(payload.into_inner()) {
        Ok(response) => Ok(ApiResponse(Status::Created, Json(response))),
        Err(e) => {
            // Log detailed error
            crate::log_error!(400, "Registration failed", &format!("{}", e));

            // ส่ง generic message ให้ client
            Err(ApiResponse(
                Status::BadRequest,
                Json(ErrorResponse {
                    error: "Registration failed".to_string(),
                }),
            ))
        }
    }
}
