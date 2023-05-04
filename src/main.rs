use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");

    let client = Client::builder()
        .default_headers(headers_from_api_key(api_key))
        .build()?;

    loop {
        let mut input = String::new();
        print!("You: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let prompt = format!("User: {}\nChatbot:", input.trim());
        let response = generate_response(&client, &prompt).await?;

        println!("Chatbot: {}", response);
    }
}

fn headers_from_api_key(api_key: String) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers
}

#[derive(Serialize)]
struct OpenAIRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    max_tokens: i32,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

async fn generate_response(
    client: &Client,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    
    let chat_messages = vec![
        ChatMessage {
            role: "system",
            content: "You are ChatGPT, a large language model trained by OpenAI.",
        },
        ChatMessage {
            role: "user",
            content: prompt,
        },
    ];

    let req = OpenAIRequest {
        model: "gpt-3.5-turbo",
        messages: chat_messages,
        max_tokens: 200,
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&req)
        .send()
        .await?;

    let res_text = res.text().await?;

    match serde_json::from_str::<OpenAIResponse>(&res_text) {
        Ok(response) => {
            let response_text = response.choices
                .get(0)
                .map(|c| c.message.content
                .trim())
                .unwrap_or("");
            Ok(response_text.to_string())
        }
        Err(e) => {
            eprintln!("Error deserializing response: {:?}", e);
            eprintln!("Raw API response: {}", res_text);
            Err(e.into())
        }
    }
}

