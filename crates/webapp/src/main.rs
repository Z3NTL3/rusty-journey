use std::{env, path::Path};

use axum::{body::Body, extract::Request, handler::Handler, http::StatusCode, middleware::{self, Next}, response::{IntoResponse, Response}, routing::get, serve::Serve, Extension, Json, Router};
use serde::{self,Deserialize, Serialize};
use thiserror::Error;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tower_http::services::ServeDir;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Oops, something went wrong!")]
    Unknown,
    #[error("Request payload has not been satisfied")]
    RequestPayload
}

#[derive(Serialize)]
struct GlobalErrResponse {
    message: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let res = axum::Json(GlobalErrResponse{
            message: format!("{self}")
        });

        match &self {
            AppError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                res
            ).into_response(),

            AppError::RequestPayload => (
                StatusCode::BAD_REQUEST, 
                res
            ).into_response(),
        }
    }
}

async fn somedumbstuff() -> Result<(), AppError> {
    Err(AppError::RequestPayload)
}

async fn pass_some_data(mut req: Request, next: Next) -> axum::response::Result<Response> {
    let data = req.headers().get("X-Data").ok_or(AppError::RequestPayload)?
        .to_str()
        .map_err(|err| {
            println!("got err: {err}");
            AppError::RequestPayload
        })?
    .to_owned();

    req.extensions_mut().insert(data);
    Ok(next.run(req).await)
}

// streams file
async fn handler(data: Extension<String>) -> axum::response::Result<Response> {
    println!("data from middleware {}", data.0);

    let file = File::open("test.mp4").await.map_err(|err| {
        println!("got err: {err}");
        AppError::Unknown
    })?;
    
    let body = Body::from_stream(ReaderStream::new(file));
    let res = Response::builder()
        .header("Content-Type", "video/mp4")
        .body(body).map_err(|err| {
            println!("got err: {err}");
            AppError::Unknown
        })?;

    Ok(res)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() {
    let layered_handler = handler.layer(middleware::from_fn(pass_some_data));
    let app = Router::new()
        .route("/", get(layered_handler))
        .nest_service("/static", ServeDir::new("assets"))
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}