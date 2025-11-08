use lambda_http::{http::Method, tracing, Body, Error, Request, Response};
use crate::handlers::create_interest;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let method = event.method();
    let path = event.uri().path();

    tracing::info!("Incoming request: {} {}", method, path);

    match (method, path) {
        (&Method::POST, "/interests") => create_interest::handler(event).await,
        _ => {
            tracing::info!("Route not found: {} {}", method, path);
            Ok(Response::builder()
                .status(404)
                .body("Not Found".into())
                .expect("Failed to render response"))
        }
    }
}
