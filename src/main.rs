use axum::{Router, routing::{get, post}};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use log::info;
use dotenv::dotenv;

mod handlers;
mod models;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let app = Router::new()
        .route("/", get(handlers::serve_form))
        .route("/describe_image", post(handlers::describe_image))
        .nest_service("/static", ServeDir::new("static"));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    info!("Server starting on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}