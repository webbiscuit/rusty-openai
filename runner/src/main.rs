use dotenv::dotenv;
use openai_api_client::models::list_models;
use std::io::Write;
use std::{env, io};

// use openapi::apis::configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");

    // let data = list_models(&api_key).await?;
    // println!("Data: {:#?}", data);

    // let request = openai_api_client::completion::CreateCompletionRequest {
    //     model: "text-davinci-003".to_string(),
    //     prompt: Some("Write a tagline for an ice cream shop.".to_string()),
    // };

    // let data2 = openai_api_client::completion::create_completion(&api_key, request).await?;
    // println!("Data: {:#?}", data2);

    loop {
        print!("YOU > ");
        io::stdout().flush().unwrap();

        let mut prompt = String::new();
        std::io::stdin().read_line(&mut prompt)?;
        prompt = prompt.trim().to_string();

        //println!("Bot: {}", prompt);

        let request = openai_api_client::completion::CreateCompletionRequest {
            model: "text-davinci-003".to_string(),
            prompt: Some(prompt),
        };
        let response = openai_api_client::completion::create_completion(&api_key, request).await?;
        println!("BOT > {}", response.choices[0].text);
    }

    Ok(())
}
