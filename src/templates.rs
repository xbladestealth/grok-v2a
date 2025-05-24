use axum::response::{Html, IntoResponse};

pub fn render_index() -> impl IntoResponse {
    Html(std::fs::read_to_string("templates/index.html").expect("Failed to read index.html"))
}