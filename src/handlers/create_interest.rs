use crate::{
    db::add_interest,
    models::{Interest, InterestFormData}
};
use lambda_http::{tracing, Body, Error, Request, Response};

pub async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let body = match event.body() {
        Body::Text(text) => text,
        Body::Empty => {
            tracing::info!("Request body is missing");
            return Ok(Response::builder()
                .status(400)
                .body("Request body is missing".into())
                .expect("Failed to render response"));
        }
        _ => {
            tracing::info!("Invalid request body");
            return Ok(Response::builder()
                .status(400)
                .body("Invalid request body".into())
                .expect("Failed to render response"));
        }
    };

    let interest_data: InterestFormData = match serde_json::from_str(body) {
        Ok(data) => data,
        Err(error) => {
            tracing::error!("Invalid JSON. Error: {}", error);
            return Ok(Response::builder()
                .status(400)
                .body("Invalid JSON".into())
                .expect("Failed to render response"));
        }
    };

    let interest = Interest {
        id: uuid::Uuid::new_v4().to_string(),
        name: interest_data.name.clone(),
        email: interest_data.email.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    if let Err(e) = add_interest(interest.clone()).await {
        tracing::error!("Failed to add interest. Error: {}", e);
        return Ok(Response::builder()
            .status(500)
            .body("Internal Server Error".into())
            .expect("Failed to render response"));
    }

    tracing::info!("Interest added successfully: {:#?}", interest);

    Ok(Response::builder()
        .status(201)
        .body(Body::Empty)
        .expect("Failed to render response"))
}
