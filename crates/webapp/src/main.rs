use std::{future::Future, pin::Pin, sync::Arc, task::Poll};

use axum::{body::Body, extract::{Request, State}, handler::{Handler, HandlerWithoutStateExt}, http::StatusCode, middleware::{self, Next}, response::{Html, IntoResponse, Response}, routing::get, Extension, Router};
use minijinja::{context, Environment};
use serde::{self, Serialize};
use thiserror::Error;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tower::{MakeService, Service};
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct GlobalErrResponse {
    message: String
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Oops, something went wrong!")]
    Unknown,
    #[error("Request payload has not been satisfied")]
    RequestPayload
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

async fn handler_404(template: State<Arc<Environment<'_>>>) -> axum::response::Result<Response> {
    let template = template.get_template("error.html").map_err(|e|{
        println!("{e}");
        AppError::Unknown
    })?;
    
    let res = template.render(context! {text => "hello world"}).map_err(|e|{
        println!("{e}");
        AppError::Unknown
    })?;
    Ok(Html(res).into_response())
}

#[tokio::main]
async fn main() {
    let mut templates = Environment::new();
    templates.set_loader(minijinja::path_loader("crates/webapp/views"));

    let service_404 = handler_404.with_state(templates.clone().into());
    let assets = ServeDir::new("assets").not_found_service(service_404.clone());

    let layered_handler = handler.layer(middleware::from_fn(pass_some_data));
    let app = Router::new()
        .route("/", get(layered_handler))
        .nest_service("/static", assets)
        .fallback_service(service_404)
        .with_state(templates);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}