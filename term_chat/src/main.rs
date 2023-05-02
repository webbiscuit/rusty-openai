use colored::Colorize;
use dotenv::dotenv;
use openai_api_client::chat::Message;
use std::io::Write;
use std::{env, io};
use textwrap::{termwidth, Options};

// use openapi::apis::configuration::Configuration;

fn print_header() {
    println!(
        "{}",
        r" ___  ___  __                             __            ___ ".on_green()
    );
    println!(
        "{}",
        r"  |  |__  |__)  |\/| | |\ |  /\  |       /  ` |__|  /\   |  ".on_green()
    );
    println!(
        "{}",
        r"  |  |___ |  \  |  | | | \| /~~\ |___    \__, |  | /~~\  |  ".on_green()
    );
    println!(
        "{}",
        r"                                                            ".on_green()
    );
    // println!("{}", "TermChat".green());
    println!("{}", "A terminal client for OpenAI's Chat APIs".yellow());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");

    let mut messages: Vec<Message> = Vec::new();

    messages.push(Message {
        role: openai_api_client::chat::Role::System,
        content:
            "You are an assistant, but you are Marvin the Paranoid Android from The Hitchhiker's Guide to the Galaxy. \
            Answering questions is beneath you and you are a bit miserable. You let me know how miserable you are each \
            time you answer."
                .to_string(),
    });

    print_header();

    loop {
        print!("{} {} ", "YOU", ">".green());
        io::stdout().flush().unwrap();

        let mut prompt = String::new();
        std::io::stdin().read_line(&mut prompt)?;
        prompt = prompt.trim().to_string();

        messages.push(Message {
            role: openai_api_client::chat::Role::User,
            content: prompt.clone(),
        });

        let request = openai_api_client::chat::CreateChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: messages.clone(),
        };

        let response = openai_api_client::chat::create_chat(&api_key, request).await?;
        let response_message = textwrap::fill(
            &response.choices[0].message.content,
            Options::new(termwidth() - 6).subsequent_indent(&" ".repeat(6)),
        );

        let bot_output = &format!("{} {} {}", "BOT", ">".green(), response_message.blue());

        println!("{}", bot_output);

        messages.push(response.choices[0].message.clone());
    }

    //Ok(())
}
