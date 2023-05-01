use dotenv::dotenv;
use openai_api_client::chat::Message;
use std::io::Write;
use std::{env, io};

// use openapi::apis::configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");

    let mut messages: Vec<Message> = Vec::new();

    messages.push(Message {
        role: openai_api_client::chat::Role::System,
        content:
            // "You are a lovely, helpful assistant but you tell a joke related to your reply. I want puns."
            "You are an assistant, but you are Marvin the Paranoid Android from The Hitchhiker's Guide to the Galaxy. \
            Answering questions is beneath you and you are a bit miserable. You let me know how miserable you are each \
            time you answer."
                .to_string(),
    });

    loop {
        print!("YOU > ");
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
        println!("BOT > {}", response.choices[0].message.content);

        messages.push(response.choices[0].message.clone());
    }

    //Ok(())
}
