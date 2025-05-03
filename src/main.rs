use anyhow::{Context, Result};
use async_openai::{
    Client, 
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, 
        CreateChatCompletionRequestArgs,
        Role,
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        ChatCompletionRequestAssistantMessageArgs,
    }
};
use dotenv::dotenv;
use futures::StreamExt;
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

/// Create a system message to initialize the conversation
fn create_system_message() -> Result<ChatCompletionRequestMessage> {
    let system_message = ChatCompletionRequestSystemMessageArgs::default()
        .content("You are a helpful, friendly assistant. Provide clear and concise responses to help the user.")
        .build()?;
    
    Ok(system_message.into())
}

/// Create a user message from input text
fn create_user_message(content: &str) -> Result<ChatCompletionRequestMessage> {
    let user_message = ChatCompletionRequestUserMessageArgs::default()
        .content(content)
        .build()?;
    
    Ok(user_message.into())
}

/// Create an assistant message
fn create_assistant_message(content: &str) -> Result<ChatCompletionRequestMessage> {
    let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
        .content(content)
        .build()?;
    
    Ok(assistant_message.into())
}

/// Send a message to the API and stream the response
async fn send_message_streaming(
    client: &Client<OpenAIConfig>,
    model: &str,
    messages: &[ChatCompletionRequestMessage],
) -> Result<String> {
    // Create the API request with streaming enabled
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages.to_vec())
        .stream(true)
        .build()?;
    
    // Send the request and get a stream of responses
    let mut stream = client.chat().create_stream(request).await?;
    
    print!("AI> ");
    io::stdout().flush().context("Failed to flush stdout")?;
    
    let mut full_response = String::new();
    
    // Process each chunk as it arrives
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                // Extract content from the response delta
                if let Some(delta) = response.choices.first() {
                    if let Some(content) = &delta.delta.content {
                        // Print the content fragment immediately
                        print!("{}", content);
                        io::stdout().flush().context("Failed to flush stdout")?;
                        
                        // Collect the complete response
                        full_response.push_str(content);
                    }
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Error receiving response: {}", e));
            }
        }
    }
    
    println!(); // End the line after the complete response
    
    Ok(full_response)
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
    
    // Initialize conversation history with a system message
    let mut messages = vec![create_system_message()?];
    
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
        
        // Create a message from user input and add to history
        let user_message = create_user_message(&user_input)?;
        messages.push(user_message);
        
        // Send message to API and stream the response
        match send_message_streaming(&client, &config.model, &messages).await {
            Ok(response) => {
                // Add the assistant's response to conversation history
                let assistant_message = create_assistant_message(&response)?;
                messages.push(assistant_message);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    
    Ok(())
}
