use axum::{extract::Request, handler::Handler, http::StatusCode, middleware::{self, Next}, response::{IntoResponse, Response}, routing::get, Extension, Json, Router};
use serde::{self,Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Oops, something went wrong!")]
    Unknown,
    #[error("Request payload has not been satisfied")]
    RequestPayload
}

#[derive(Deserialize, Serialize)]
struct GlobalErrResponse {
    message: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let err = format!("{self}");

        match &self {
            AppError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                axum::Json(GlobalErrResponse{
                    message: err
                })
            ).into_response(),

            AppError::RequestPayload => (
                StatusCode::BAD_REQUEST, 
                axum::Json(GlobalErrResponse{
                    message: err
                })
            ).into_response(),
        }
    }
}

async fn somedumbstuff() -> Result<(), AppError> {
    Err(AppError::RequestPayload)
}

async fn pass_some_data(mut req: Request, next: Next) -> axum::response::Result<Response> {
    let mut data = String::default();
    match req.headers().get("X-Data") {
        Some(header) => {
            data = header.to_str().unwrap_or_default().to_owned();
        },
        None => Err(AppError::RequestPayload)?,
    }

    req.extensions_mut().insert(data);
    Ok(next.run(req).await)
}

async fn handler(data: Extension<String>) -> String {
    data.0
}

#[tokio::main]
async fn main() {
    let layered_handler = handler.layer(middleware::from_fn(pass_some_data));
    let app = Router::new()
        .route("/", get(layered_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}