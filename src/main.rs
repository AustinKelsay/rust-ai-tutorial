use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

struct Config {
    api_key: String,
    base_url: String,
    model: String,
}

impl Config {
    fn from_env() -> Result<Self> {
        // Load environment variables from .env file if it exists
        dotenv().ok();

        // Get the API key (required)
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY is required. Please set it in your environment or .env file.")?;

        // Get the base URL (optional, has default)
        let base_url = env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        // Get the model name (optional, has default)
        let model = env::var("MODEL_NAME")
            .unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

        Ok(Config {
            api_key,
            base_url,
            model,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to CLI Chat!");
    println!("A simple command-line chat application.");
    
    // Load configuration from environment variables
    let config = Config::from_env()
        .context("Failed to load configuration")?;
    
    println!("Configuration loaded successfully.");
    println!("Using model: {}", config.model);
    println!("Type your messages after setup is complete.");
    
    Ok(())
}
