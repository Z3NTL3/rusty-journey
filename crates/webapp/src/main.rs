use axum::{extract::Request, handler::Handler, middleware::{self, Next}, response::IntoResponse, routing::get, Extension, Router};
use tower::Service;

async fn pass_some_data(mut req: Request, mut next: Next) -> impl IntoResponse {
    let mut data = String::default();

    match req.headers().get("X-Data") {
        Some(header) => {
            data = header.to_str().unwrap_or_default().to_owned();
        },
        None => {
            data = "none".to_string();
        },
    }

    req.extensions_mut().insert(data);
    next.call(req).await
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