use lambda_http::{tracing, Body, Error, Request, Response};

use crate::{
    db::add_interest,
    models::{Interest, InterestFormData}
};

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // check request method and uri
    if event.method() != "POST" || event.uri().path() != "/interests" {
        tracing::info!("Method Not Allowed. Method: {}, Path: {}", event.method(), event.uri().path());
        return Ok(Response::builder()
            .status(405)
            .body("Method Not Allowed".into())
            .expect("Failed to render response"));
    }

    // get request body
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

    // parse body data
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

    // insert data into database
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

    // create response
    Ok(Response::builder()
        .status(201)
        .body(Body::Empty)
        .expect("Failed to render response"))
}
