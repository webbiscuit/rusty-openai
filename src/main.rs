use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Prompt {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: u32,
}

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");
    let client = Client::new();

    let prompt = Prompt {
        model: "text-davinci-003".to_string(),
        prompt: "Is this okay?".to_string(),
        max_tokens: 7,
        temperature: 0,
    };

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&prompt)
        .send()
        .await?;

    println!("Response: {:?}", response);
    println!("Response: {:?}", response.status());

    if response.status() != 200 {
        println!("Response: {}", response.text().await?);
        return Ok(());
    }

    //println!("Response: {:?}", response.text().await?);

    let api_response: ApiResponse = response.json().await?;
    let completion = api_response.choices.get(0).unwrap().text.trim();

    println!("Generated text: {}", completion);

    Ok(())
}
