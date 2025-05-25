use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    pub detail: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub message: MessageContent,
}

#[derive(Serialize, Deserialize)]
pub struct MessageContent {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}