use axum::{ routing::{get, post}, Router, serve };
mod fetch;
mod plot;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/fetch", post(fetch::Fetch))
        .route("/plot", post(plot::Plot));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "This is a kelly criterion plotter for stock symbols."
}
