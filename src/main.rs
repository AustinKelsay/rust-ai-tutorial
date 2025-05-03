use anyhow::{Context, Result};
use async_openai::{Client, config::OpenAIConfig};
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};

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
}

/// Read a line of user input with a custom prompt
fn read_input(prompt: &str) -> Result<String> {
    // Print the prompt and flush to ensure it appears before waiting for input
    print!("{}", prompt);
    io::stdout().flush().context("Failed to flush stdout")?;
    
    // Read the input line
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("Failed to read input")?;
    
    // Trim whitespace and return
    Ok(input.trim().to_string())
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
    
    // Main interaction loop
    loop {
        // Get user input with a prompt
        let user_input = read_input("You> ")?;
        
        // Check for exit command
        if user_input.to_lowercase() == "/exit" {
            println!("Exiting chat. Goodbye!");
            break;
        }
        
        // Skip empty messages
        if user_input.trim().is_empty() {
            continue;
        }
        
        // Process the user input (for now, just echo it back)
        println!("AI> You said: {}", user_input);
        
        // In the future, we'll send this to the OpenAI API and display the response
    }
    
    Ok(())
}
