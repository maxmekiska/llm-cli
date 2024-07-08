/*
    Simple chat routine to interact with OpenRouters's GPT models.
    The routine expects the LLM_API_KEY to be set in ENV.

    Default config:
        model: "meta-llama/llama-3-8b-instruct:free"
        temperature: 0.7
        max_tokens: 800
        top_p: 0.8
        n: 1

    There are three special commands:
        1. exit: exits the chat
        2. clear: clears the full chat history/memory
        3. undo: clears the last user and agent prompts
*/
use clap::{Parser, Subcommand};

mod chatroutine;
mod cliutils;
mod openaiapi;

#[derive(Parser)]
#[command(name = "llm-cli")]
#[command(author = "Max Mekiska. <maxmekiska@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "CLI to interact with OpenRouters's GPT models.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Chat {
        #[arg(short = 'm', long, default_value_t = String::from("meta-llama/llama-3-8b-instruct:free"), help = "Model to use.")]
        model: String,

        #[arg(short = 't', long, default_value_t = 0.7, help = "Model temperature.")]
        temperature: f64,

        #[arg(
            short = 'x',
            long,
            default_value_t = 800,
            help = "Maximum tokens the model should generate."
        )]
        max_tokens: i32,

        #[arg(short = 'p', long, default_value_t = 0.8, help = "Top P value.")]
        top_p: f64,

        #[arg(
            short = 'n',
            long,
            default_value_t = 1,
            help = "Number of completions to generate."
        )]
        n: i32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Chat {
            model,
            temperature,
            max_tokens,
            top_p,
            n,
        }) => chatroutine::run_chat(model, *temperature, *max_tokens, *top_p, *n).await,
        None => Ok(()),
    }
}
