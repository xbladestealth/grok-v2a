use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    extract::Json as JsonExtract,
};
use axum_extra::extract::Multipart;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use log::{error, info};
use reqwest::Client;
use serde_json::json;
use std::env;

use crate::models::{ApiError, CompletionResponse, GenerateScriptRequest};
use crate::templates;

pub async fn serve_form() -> impl IntoResponse {
    templates::render_index()
}

pub async fn generate_python_script(Json(payload): JsonExtract<GenerateScriptRequest>) -> impl IntoResponse {
    let user_prompt = payload.prompt;
    if user_prompt.is_empty() {
        error!("No prompt provided in request");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No prompt provided" })),
        ));
    }

    let api_key = env::var("XAI_API_KEY").map_err(|e| {
        error!("XAI_API_KEY not set: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Missing API key: {}", e) })),
        )
    })?;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create HTTP client: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to create HTTP client: {}", e) })),
            )
        })?;

    let request_body = json!({
        "model": "grok-3",
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": format!("Generate a Python script for the following task: {}", user_prompt)
                    }
                ]
            }
        ],
        "temperature": 0.0,
    });

    info!(
        "Request payload: {}",
        serde_json::to_string_pretty(&request_body).map_err(|e| {
            error!("Failed to serialize request payload: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to serialize request: {}", e) })),
            )
        })?
    );

    let mut retries = 3;
    let response = loop {
        let res = client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
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
            let delay_secs = 1 << (3 - retries);
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Request failed after retries: {}", e) })),
        )
    })?;

    let status = response.status();
    let body = response.text().await.map_err(|e| {
        error!("Failed to read response body: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to read response body: {}", e) })),
        )
    })?;
    info!("API response status: {}, body: {}", status, body);

    if !status.is_success() {
        let error_msg = match serde_json::from_str::<ApiError>(&body) {
            Ok(api_error) => api_error.error,
            Err(_) => body.clone(),
        };
        error!("API error: status {}, body: {}", status, error_msg);
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({ "error": format!("API error: status {}, body: {}", status, error_msg) })),
        ));
    }

    let api_response = serde_json::from_str::<CompletionResponse>(&body).map_err(|e| {
        error!("JSON parse error: {}. Response body: {}", e, body);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("JSON parse error: {}", e) })),
        )
    })?;

    let script = api_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .unwrap_or("No script generated".to_string());

    info!("Generated Python script: {}", script);

    Ok(Json(json!({
        "script": script
    })))
}

pub async fn describe_image(mut multipart: Multipart) -> impl IntoResponse {
    let mut file_data = Vec::new();
    let mut user_prompt = String::new();
    let mut content_type: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to read multipart field: {}", e) })),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "image" => {
                content_type = field.content_type().map(|ct| ct.to_string());
                if let Some(ct) = &content_type {
                    if !ct.starts_with("image/jpeg") && !ct.starts_with("image/png") {
                        error!("Invalid content type: {}", ct);
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(json!({ "error": "Only JPEG and PNG images are supported" })),
                        ));
                    }
                } else {
                    error!("Missing content type in multipart form");
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": "Missing content type" })),
                    ));
                }
                let data = field.bytes().await.map_err(|e| {
                    error!("Failed to read file data: {}", e);
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": format!("Failed to read file data: {}", e) })),
                    )
                })?;
                file_data.extend_from_slice(&data);
            }
            "prompt" => {
                let text = field.text().await.map_err(|e| {
                    error!("Failed to read prompt text: {}", e);
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": format!("Failed to read prompt text: {}", e) })),
                    )
                })?;
                user_prompt = text;
            }
            _ => {
                error!("Unexpected field in multipart form: {}", name);
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("Unexpected field: {}", name) })),
                ));
            }
        }
    }

    if file_data.is_empty() {
        error!("No file uploaded in multipart form");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No file uploaded" })),
        ));
    }
    if user_prompt.is_empty() {
        error!("No prompt provided in multipart form");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No prompt provided" })),
        ));
    }
    if file_data.len() > 5 * 1024 * 1024 {
        error!("Image size exceeds 5MB limit: {} bytes", file_data.len());
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Image size exceeds 5MB limit" })),
        ));
    }

    let image_base64 = BASE64.encode(&file_data);
    info!("Base64 image length: {} bytes", image_base64.len());

    let decoded = BASE64.decode(&image_base64).map_err(|e| {
        error!("Invalid base64 data: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Invalid image data: {}", e) })),
        )
    })?;
    info!("Decoded image size: {} bytes", decoded.len());

    let api_key = env::var("XAI_API_KEY").map_err(|e| {
        error!("XAI_API_KEY not set: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Missing API key: {}", e) })),
        )
    })?;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create HTTP client: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to create HTTP client: {}", e) })),
            )
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
                            "url": format!(
                                "data:image/{};base64,{}", 
                                if content_type.as_ref().unwrap().starts_with("image/jpeg") { "jpeg" } else { "png" }, 
                                image_base64
                            ),
                            "detail": "high"
                        }
                    },
                    {
                        "type": "text",
                        "text": user_prompt
                    }
                ]
            }
        ],
        "temperature": 0.0,
    });

    info!(
        "Request payload: {}",
        serde_json::to_string_pretty(&request_body).map_err(|e| {
            error!("Failed to serialize request payload: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to serialize request: {}", e) })),
            )
        })?
    );

    let mut retries = 3;
    let response = loop {
        let res = client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
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
            let delay_secs = 1 << (3 - retries);
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Request failed after retries: {}", e) })),
        )
    })?;

    let status = response.status();
    let body = response.text().await.map_err(|e| {
        error!("Failed to read response body: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to read response body: {}", e) })),
        )
    })?;
    info!("API response status: {}, body: {}", status, body);

    if !status.is_success() {
        let error_msg = match serde_json::from_str::<ApiError>(&body) {
            Ok(api_error) => api_error.error,
            Err(_) => body.clone(),
        };
        error!("API error: status {}, body: {}", status, error_msg);
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({ "error": format!("API error: status {}, body: {}", status, error_msg) })),
        ));
    }

    let api_response = serde_json::from_str::<CompletionResponse>(&body).map_err(|e| {
        error!("JSON parse error: {}. Response body: {}", e, body);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("JSON parse error: {}", e) })),
        )
    })?;

    let description = api_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .unwrap_or("No description available".to_string());

    info!("Extracted description: {}", description);

    Ok(Json(json!({
        "description": description,
        "image_url": format!(
            "data:image/{};base64,{}", 
            if content_type.as_ref().unwrap().starts_with("image/jpeg") { "jpeg" } else { "png" }, 
            image_base64
        )
    })))
}