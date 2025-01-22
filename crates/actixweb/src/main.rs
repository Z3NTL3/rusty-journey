use actix_web::{get, App, HttpRequest, HttpServer};
use thiserror::Error;
use tracing::{instrument, Level};
use tracing_subscriber::fmt::{format::FmtSpan, time::ChronoLocal};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error:{something}")]
    Unknown{something: &'static str},
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut builder = actix_web::HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST);
        match self {
            AppError::Unknown { something } => builder.body(*something),
        }
    }
} 

#[instrument(
    name = "app::get", 
    level = Level::DEBUG, 
    skip(req)
    fields(path = req.uri().path().to_string())
)]
#[get("/")]

async fn index(req: HttpRequest) -> Result<String, AppError> {
    tracing::info!("serving req");
    Err(AppError::Unknown { something: "oops br" })
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_span_events(FmtSpan::NEW)
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".into()))
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 2000)).expect("failed starting server")
    .run()
    .await.unwrap();
}