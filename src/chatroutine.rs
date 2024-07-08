use std::env;
use termimad::MadSkin;

use crate::cliutils::{get_user_input, special_commands};
use crate::openaiapi::{send_request, OpenAIMessage, OpenAIRequest};

pub async fn run_chat(
    model: &str,
    temperature: f64,
    max_tokens: i32,
    top_p: f64,
    n: i32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let openai_api_key: String = env::var("LLM_API_KEY").unwrap();
    let uri = "https://openrouter.ai/api/v1/chat/completions";

    let mut conversation_history: Vec<OpenAIMessage> = Vec::new();

    loop {
        let user_text = get_user_input();

        let action = special_commands(&user_text, &mut conversation_history);

        if action == 1 {
            break;
        } else if action == 2 {
            continue;
        }

        let user_message = OpenAIMessage {
            role: String::from("user"),
            content: user_text.trim().to_string(),
        };

        conversation_history.push(user_message.clone());

        let openai_request = OpenAIRequest {
            model: model.to_string(),
            messages: conversation_history.clone(),
            temperature: Some(temperature),
            max_tokens: Some(max_tokens),
            top_p: Some(top_p),
            n: Some(n),
        };

        match send_request(&uri, &openai_api_key, &openai_request).await {
            Ok(response) => {
                if let Some(choice) = response.choices.get(0) {
                    let markdown_content = &choice.message.content;

                    let skin = MadSkin::default();

                    println!("\x1b[32m>\x1b[0m");
                    skin.print_text(markdown_content);

                    conversation_history.push(choice.message.clone());
                }
            }
            Err(e) => {
                println!(
                    ">\x1b[31m<\x1b[0m Error processing request: {}. Please try again.",
                    e
                );
            }
        }
    }

    Ok(())
}
