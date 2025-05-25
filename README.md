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
### Prerequisites
- Rust (latest stable version recommended, install via [rustup](https://rustup.rs/)).
- An xAI API key (required for image analysis and script generation).

### Running the Application
1. Clone the repository:
   ```bash
   git clone https://github.com/xbladestealth/grok-v2a
2. Navigate to the project directory:
   ```bash
   cd grok-v2a
3. you can add your xAI API key to a .env file:
   ```bash
   echo "XAI_API_KEY=your-api-key" > .env
4. Build and run the application:
   ```bash
   cargo run
5. Access the application:  
   http://localhost:3000
