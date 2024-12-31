use std::sync::Arc;
use axum::{async_trait, body::Body, extract::{FromRequestParts, Request, State}, handler::Handler, http::{request::Parts, HeaderValue, StatusCode}, middleware::{self, Next}, response::{Html, IntoResponse, Response}, routing::get, Extension, Router};
use minijinja::{context, Environment};
use serde::{self, Serialize};
use thiserror::Error;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct GlobalErrResponse {
    message: String
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Oops, something went wrong!")]
    Oops,
    #[error("Request payload has not been satisfied")]
    RequestPayload
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let res = axum::Json(GlobalErrResponse{
            message: format!("{self}")
        });

        match &self {
            AppError::Oops => (
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
        AppError::Oops
    })?;
    
    let body = Body::from_stream(ReaderStream::new(file));
    let res = Response::builder()
        .header("Content-Type", "video/mp4")
        .body(body).map_err(|err| {
            println!("got err: {err}");
            AppError::Oops
        })?;
        
    Ok(res)
}

async fn handler_404(template: State<Arc<Environment<'static>>>) -> axum::response::Result<Response> {
    let template = template.get_template("error.html").map_err(|e|{
        println!("{e}");
        AppError::Oops
    })?;

    let res = template.render(context! {text => "hello world"}).map_err(|e|{
        println!("{e}");
        AppError::Oops
    })?;
    Ok(Html(res).into_response())
}


struct ExtractXData(String);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractXData
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let extracted_part = parts.headers.get("X-Data")
            .map_or(Ok(""), |v| v.to_str())
            .map_err(|e| {
                println!("got error from ExtractUserAgent extractor: {e}");
                AppError::Oops // dont expose exact error to client
            })?;

        Ok(ExtractXData(extracted_part.into()))
    }
}

async fn some_handler(x_data: ExtractXData, template: State<Arc<Environment<'static>>>) -> axum::response::Result<Response> {
    let template = template.get_template("error.html").map_err(|e|{
        println!("{e}");
        AppError::Oops
    })?;

    let res = template.render(context! {text => format!("hello, got X-Data: {}", x_data.0)}).map_err(|e|{
        println!("{e}");
        AppError::Oops
    })?;
    Ok(Html(res).into_response())
}


#[tokio::main]
async fn main() {
    let mut templates = Environment::new();
    templates.set_loader(minijinja::path_loader("crates/webapp/views"));
    templates.add_filter("test", |a: u8| {
        5 + a
    });

    let engine: Arc<Environment<'_>> = templates.into();
    let service_404 = handler_404.with_state(engine.clone());
    let assets = ServeDir::new("crates/webapp/assets").not_found_service(service_404.clone());

    let layered_handler = handler.layer(middleware::from_fn(pass_some_data));
    let app = Router::new()
        .route("/", get(layered_handler))
        .route("/x-data", get(some_handler))
        .nest_service("/static", assets)
        .fallback_service(service_404)
        .with_state(engine);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}