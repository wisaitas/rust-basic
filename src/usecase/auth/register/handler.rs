use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use validator::Validate;

use crate::DbPool;
use crate::domain::repository::user::UserRepositoryImpl;
use crate::usecase::auth::register::dto::{ErrorResponse, RegisterRequest};
use crate::usecase::auth::register::service::RegisterService;

pub async fn register_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Response {
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

        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: error_message,
            }),
        )
            .into_response();
    }

    // Get connection from pool
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database connection error: {}", e),
                }),
            )
                .into_response();
        }
    };

    // Create repository and service
    let user_repo = UserRepositoryImpl { conn };
    let mut service = RegisterService::new(user_repo);

    // Process registration
    match service.register(payload) {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}
