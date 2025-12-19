use colored::Colorize;
use dotenvy::dotenv;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::io::{self, Write};
use termimad::MadSkin;
use tokio;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: Delta,
}

#[derive(Debug, Deserialize)]
struct Delta {
    content: Option<String>,
}

// ============================================================================
// Configuration
// ============================================================================

struct Config {
    api_key: String,
    model: String,
}

impl Config {
    fn from_env() -> Self {
        dotenv().ok();
        
        let api_key = env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set in .env file");
        
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-5".to_string());
        
        Config { api_key, model }
    }
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[tokio::main]
async fn main() {
    println!("Termline - Terminal Chat Assistant");
    println!("Setting up...\n");
    
    let config = Config::from_env();
    println!("Using model: {}", config.model);
    println!("Type 'exit' or ':q' to quit\n");
    
    // Initialize conversation with system message
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: "You are a chat assistant living in my computer terminal. I am probably trying to get quick answers.".to_string(),
    }];
    
    // TODO: Implement REPL loop
    // TODO: Implement OpenAI API client with streaming
    // TODO: Implement terminal UI with colored output
    // TODO: Implement markdown rendering
}
