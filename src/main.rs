use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::io;
use std::io::Write;
use copypasta::{ClipboardContext, ClipboardProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");

    let client = Client::builder()
        .default_headers(headers_from_api_key(api_key))
        .build()?;

    // Clear the terminal screen
    clear_terminal();

    let mut first_iteration = true;
    let mut previous_input = String::new();
    let mut previous_response = String::new();

    loop {
        let mut input = String::new();

        if first_iteration {
            println!("What do you want to talk about?");
            println!();
            print!("You: ");
            first_iteration = false;
        } else {
            println!(); // Add a newline before the user's prompt
            print!("You: ");
        }

        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("c") {
            // Copy the last response to the clipboard
            let mut clipboard_context = ClipboardContext::new().unwrap();
            clipboard_context.set_contents(previous_response.clone()).unwrap();
            println!("Response copied to clipboard.");
            continue;
        } else if input.trim().eq_ignore_ascii_case("r") {
            // Resend the last question
            input = previous_input.clone();
        } else {
            if !previous_input.is_empty() {
                println!();
                println!("You: {}", previous_input.trim());
            }
            previous_input = input.clone();
        }

        let prompt = format!("User: {}\nQuasar:", input.trim());

        let response = generate_response(&client, &prompt).await?;
        previous_response = response.clone();

        println!("Quasar: {}", response);
        println!("Copy (c) | Regenerate (r)");
    }
}

#[cfg(target_family = "unix")]
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

#[cfg(target_family = "windows")]
fn clear_terminal() {
    let output_handle = winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
    let mut csbi: winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO = unsafe { std::mem::zeroed() };
    let csbi_ptr = &mut csbi as *mut winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO;

    if unsafe { winapi::um::wincon::GetConsoleScreenBufferInfo(output_handle, csbi_ptr) } == 0 {
        return;
    }

    let cells = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;
    let mut chars_written = 0u32;
    let coord = winapi::um::wincon::COORD { X: 0, Y: 0 };
    let fill_char = ' ' as u16;

    if unsafe {
        winapi::um::wincon::FillConsoleOutputCharacterW(
            output_handle,
            fill_char,
            cells,
            coord,
            &mut chars_written as *mut u32,
        )
    } == 0
    {
        return;
    }

    if unsafe {
        winapi::um::wincon::FillConsoleOutputAttribute(
            output_handle,
            csbi.wAttributes,
            cells,
            coord,
            &mut chars_written as *mut u32,
        )
    } == 0
    {
        return;
    }

    unsafe {
        winapi::um::wincon::SetConsoleCursorPosition(output_handle, coord);
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
        max_tokens: 3000,
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

