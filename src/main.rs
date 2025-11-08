use axum::{Router, routing::post};
use rust_101::create_pool;
use rust_101::usecase::auth::register::handler::register_handler;

#[tokio::main]
async fn main() {
    // Setup database connection pool
    let pool = create_pool();

    // Setup routes
    let app = Router::new()
        .route("/api/auth/register", post(register_handler))
        .with_state(pool);

    // Start server
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config::CONFIG.server.server_port))
            .await
            .unwrap();

    println!("Server running on http://localhost:3000");
    println!("Register endpoint: POST http://localhost:3000/api/auth/register");

    axum::serve(listener, app).await.unwrap();
}
