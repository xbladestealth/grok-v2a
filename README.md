# grok-v2a (GrokVisionToAct)
  
**grok-v2a** is a Rust-based web app that uses xAI's Grok-2-Vision to analyze scenes and Grok-3 to generate rover control scripts, embodying **Vision to Act**.

## Pipeline
- **Vision**: Grok-2-Vision analyzes images (e.g., `{"rock": "5m"}`).
- **Act**: Grok-3 generates control scripts (e.g., `move_forward(5)`).
- **Rust**: Simple web API with Axum for integration.

## Why grok-v2a?
- **Simple**: No complex algorithms, Grok takes care of them.
- **Fast**: Rust’s lightweight async server for speed.

## Getting Started
```bash
git clone https://github.com/xbladestealth/grok-v2a
cd grok-v2a
cargo run