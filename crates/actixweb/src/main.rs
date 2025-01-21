use serde::{Deserialize, Serialize};
use actix_web::{get, web, App, HttpServer};
use thiserror::Error;

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

#[get("/")]
async fn index() -> Result<String, AppError> {
    Err(AppError::Unknown { something: "oops br" })
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "Platform", rename_all = "SCREAMING-KEBAB-CASE")]
enum User {
    #[serde(rename_all = "PascalCase")]
    Discord{email: String},
    #[serde(other)]
    Unknown
}

#[tokio::main]
async fn main() {
    let discord_user = User::Discord { email: "yolo@gmail.com".into() };
    let serialized = serde_json::to_string(&discord_user).unwrap();

    println!("serialized: {serialized}");
    // serialized: {"Platform":"DISCORD","Email":"yolo@gmail.com"}

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                .service(index)
        )
    })
    .bind(("127.0.0.1", 2000)).expect("failed starting server")
    .run()
    .await.unwrap();
}