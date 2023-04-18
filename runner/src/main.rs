use dotenv::dotenv;
use openapi::*;
// use openapi::apis::{open_ai_api, configuration};
// use openapi::models::create_completion_request;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
// use openapi::models::CreateCompletionRequest;
// use openapi::models::CreateCompletionRequestPrompt;

#[derive(Serialize)]
struct Prompt {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
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

    // let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");
    // let client = Client::new();

    // let prompt = Prompt {
    //     model: "text-davinci-003".to_string(),
    //     prompt: "Once upon a time...".to_string(),
    //     max_tokens: 20,
    //     temperature: 0.5,
    // };

    // let response = client
    //     .post("https://api.openai.com/v1/completions")
    //     .header("Authorization", format!("Bearer {}", api_key))
    //     .json(&prompt)
    //     .send()
    //     .await?;

    // println!("Response: {:?}", response);
    // println!("Response: {:?}", response.status());

    // if response.status() != 200 {
    //     println!("Response: {}", response.text().await?);
    //     return Ok(());
    // }

    // //println!("Response: {:?}", response.text().await?);

    // let api_response: ApiResponse = response.json().await?;
    // let completion = api_response.choices.get(0).unwrap().text.trim();

    // println!("Generated text: {}", completion);

    // let mut request = create_completion_request::CreateCompletionRequest::new("model".to_string());
    // request.prompt = Some("Once upon a time...".to_string());

    // // let create_completion_request = CreateCompletionRequest {
    // //     prompt: Some("Once upon a time...".to_string()),
    // //     ..Default::default()
    // // };

    // // create_completion_request

    // // let request_prompt = create_completion_request
    // // // let prompt = CreateCompletionRequestPrompt {
    // // //     prompt: Some("Once upon a time...".to_string()),
    // // //     ..Default::default()
    // // // }
    // // let request = CreateCompletionRequest {
    // //     prompt: Some("Once upon a time...".to_string()),
    // //     // max_tokens: 20,
    // //     // temperature: 0.5,
    // //     ..Default::default()
    // // };
    // open_ai_api::create_completion(configuration, create_completion_request);

    let mut config = apis::configuration::Configuration::new();
    config.bearer_access_token = Some("sk-".to_string());

    let request_prompt = models::CreateCompletionRequestPrompt {
        prompt: Some(Some(Box::new("Once upon a time...".to_string()))),
        ..Default::default()
    };

    apis::open_ai_api::create_completion(
        &config,
        // apis::configuration::Configuration::new(),
        models::CreateCompletionRequest {
            prompt: Some(Some(Box::new("Once upon a time...".to_string()))),
            ..Default::default()
        },
    );

    Ok(())
}
