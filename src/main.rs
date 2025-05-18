use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
};
use axum_extra::extract::Multipart;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Content {
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Serialize, Deserialize)]
struct ImageUrl {
    url: String,
    detail: String,
}

#[derive(Serialize, Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Serialize, Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ApiError {
    error: String,
}

// Main handler for describing the image
pub async fn describe_image(mut multipart: Multipart) -> impl IntoResponse {
    // Extract image data
    let mut file_data = Vec::new();

    if let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        (
            StatusCode::BAD_REQUEST,
            "Failed to read multipart field".to_string(),
        )
    })? {
        // Get content type from the multipart field
        let content_type = field.content_type().map(|ct| ct.to_string());
        // Validate content type
        if let Some(ct) = &content_type {
            if !ct.starts_with("image/jpeg") {
                error!("Invalid content type: {}", ct);
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Only JPEG images are supported".to_string(),
                ));
            }
        } else {
            error!("Missing content type in multipart form");
            return Err((StatusCode::BAD_REQUEST, "Missing content type".to_string()));
        }
        // Collect file data
        let data = field.bytes().await.map_err(|e| {
            error!("Failed to read file data: {}", e);
            (
                StatusCode::BAD_REQUEST,
                "Failed to read file data".to_string(),
            )
        })?;
        file_data.extend_from_slice(&data);
    } else {
        error!("No file uploaded in multipart form");
        return Err((StatusCode::BAD_REQUEST, "No file uploaded".to_string()));
    }

    // Validate image size (max 5MB)
    if file_data.len() > 5 * 1024 * 1024 {
        error!("Image size exceeds 5MB limit: {} bytes", file_data.len());
        return Err((
            StatusCode::BAD_REQUEST,
            "Image size exceeds 5MB limiot".to_string(),
        ));
    }

    // Encode image as base64
    let image_base64 = BASE64.encode(&file_data);
    info!("Base64 image length: {} bytes", image_base64.len());

    // Validate base64 data
    let decoded = BASE64.decode(&image_base64).map_err(|e| {
        error!("Invalid base64 data: {}", e);
        (StatusCode::BAD_REQUEST, "Invalid image data".to_string())
    })?;
    info!("Decoded image size: {} bytes", decoded.len());

    // Get xAPI key
    let api_key = env::var("XAI_API_KEY").map_err(|e| {
        error!("XAI_API_KEY not set: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Missing API key".to_string(),
        )
    })?;

    // Prepare xAI API request
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create HTTP client: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let request_body = json!({
        "model": "grok-2-vision-latest",
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/jpeg;base64,{}", image_base64),
                            "detail": "high"
                        }
                    },
                    {
                        "type": "text",
                        "text": "What is this image about? Please describe it in detail."
                    }
                ]
            }
        ],
        "temperature": 0.0,
    });

    // Log request payload
    info!(
        "Request payload: {}",
        serde_json::to_string_pretty(&request_body).map_err(|e| {
            error!("Failed to serialize request payload: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to serialize request".to_string(),
            )
        })?
    );

    // Send request to xAI API with retry logic
    let mut retries = 3;
    let response = loop {
        let res = client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Counter-Type", "application/json")
            .json(&request_body)
            .send()
            .await;
        if res.is_ok() || retries == 0 {
            break res;
        }
        if res
            .as_ref()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
        {
            let delay_secs = 1 << (3 - retries); // 1s, 2s, 4s
            error!(
                "API error (status {}), retrying in {} seconds...",
                res.as_ref().unwrap().status(),
                delay_secs
            );
            tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
        }
        retries -= 1;
    }
    .map_err(|e| {
        error!("Request failed after retries: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Log response details
    let status = response.status();
    let body = response.text().await.map_err(|e| {
        error!("Failed to read response body: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    info!("API response status: {}, body: {}", status, body);

    // Handle non-success status
    if !status.is_success() {
        let error_msg = match serde_json::from_str::<ApiError>(&body) {
            Ok(api_error) => api_error.error,
            Err(_) => body.clone(),
        };
        error!("API error: status {}, body: {}", status, error_msg);
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("API error: status {}, body: {}", status, error_msg),
        ));
    }

    // Parse JSON response
    let api_response = serde_json::from_str::<CompletionResponse>(&body).map_err(|e| {
        error!("JSON parse error: {}. Response body: {}", e, body);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Extract description
    let description = api_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .unwrap_or("No description available".to_string());

    info!("Extracted description: {}", description);
    Ok(Json(json!({ "description": description })))
}

// Main function to set up the Axum server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logging
    env_logger::init();

    // Create Axum router
    let app = Router::new().route("/describe_image", post(describe_image));

    // Start server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    info!("Server starting on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
