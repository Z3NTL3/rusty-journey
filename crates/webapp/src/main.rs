use axum::{body::Body, extract::Request, handler::Handler, http::StatusCode, middleware::{self, Next}, response::{IntoResponse, Response}, routing::get, Extension, Json, Router};
use serde::{self,Deserialize, Serialize};
use thiserror::Error;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

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

// streams file
async fn handler(data: Extension<String>) -> axum::response::Result<Response> {
    println!("data we got from the middleware: {}", data.0);
    let file = File::open("test.mp4").await;
    
    match file {
        Ok(f) => {
            let body = Body::from_stream(ReaderStream::new(f));
            let res = Response::builder()
                .header("Content-Type", "video/mp4")
                .body(body);

            match res {
                Ok(r) => Ok(r),
                Err(e) => {
                    println!("some error: {}", e);
                    Err(AppError::Unknown)?
                },
            }
        },
        Err(err) => {
            println!("{err}");
            Err(AppError::Unknown)?
        },
    }
}

#[tokio::main]
async fn main() {
    let layered_handler = handler.layer(middleware::from_fn(pass_some_data));
    let app = Router::new()
        .route("/", get(layered_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}