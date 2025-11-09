use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub struct ErrorLogger;

#[rocket::async_trait]
impl Fairing for ErrorLogger {
    fn info(&self) -> Info {
        Info {
            name: "Error Logger",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Log errors (4xx, 5xx status codes)
        if response.status().code >= 400 {
            let status = response.status();
            let method = request.method();
            let uri = request.uri();
            let reason = status.reason().unwrap_or("Unknown");

            // Try to read response body for error details
            let body = response.body_mut();
            let mut body = body.take(); // take() return Body directly, not Option

            use rocket::tokio::io::AsyncReadExt;
            let mut vec = Vec::new();

            match body.read_to_end(&mut vec).await {
                Ok(_) => {
                    match String::from_utf8(vec.clone()) {
                        Ok(body_str) => {
                            eprintln!(
                                "[ERROR] {} {} -> {} {} | Body: {}",
                                method, uri, status.code, reason, body_str
                            );
                            // Put body back
                            use std::io::Cursor;
                            response.set_sized_body(vec.len(), Cursor::new(vec));
                        }
                        Err(_) => {
                            eprintln!(
                                "[ERROR] {} {} -> {} {} | Body: <binary data>",
                                method, uri, status.code, reason
                            );
                            // Put body back even if not UTF-8
                            use std::io::Cursor;
                            response.set_sized_body(vec.len(), Cursor::new(vec));
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "[ERROR] {} {} -> {} {} | Failed to read body: {}",
                        method, uri, status.code, reason, e
                    );
                }
            }
        }
    }
}
