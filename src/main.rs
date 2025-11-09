use rocket::{Build, Rocket, routes};
use rust_basic::create_pool;
use rust_basic::middleware::error_logger::ErrorLogger;
use rust_basic::usecase::auth::register::handler::register_handler;

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    let pool = create_pool();

    rocket::build()
        .manage(pool)
        .mount("/api/v1", routes![register_handler])
        .attach(ErrorLogger) // เพิ่ม error logger middleware
        .configure(
            rocket::Config::figment()
                .merge(("port", rust_basic::config::CONFIG.server.server_port))
                .merge(("address", "0.0.0.0")),
        )
}
