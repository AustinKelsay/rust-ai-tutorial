use anyhow::{Context, Result};
use async_openai::{Client, config::OpenAIConfig};
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

fn create_openai_client(config: &Config) -> Client<OpenAIConfig> {
    // Configure the client with the API key and base URL
    let openai_config = OpenAIConfig::new()
        .with_api_key(&config.api_key)
        .with_api_base(&config.base_url);
    
    // Create a new client with the configuration
    Client::with_config(openai_config)
}

fn print_welcome_message(model: &str) {
    println!("\n========================================");
    println!("🤖 Welcome to CLI Chat!");
    println!("========================================");
    println!("Connected to OpenAI API successfully.");
    println!("Using model: {}", model);
    println!("\nHow to use:");
    println!("- Type your message and press Enter to send");
    println!("- Type '/exit' to end the conversation");
    println!("========================================\n");
    println!("Chat is ready! Type your message:");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment variables
    let config = Config::from_env()
        .context("Failed to load configuration")?;
    
    // Create the OpenAI client
    let client = create_openai_client(&config);
    
    // Display welcome message
    print_welcome_message(&config.model);
    
    Ok(())
}
