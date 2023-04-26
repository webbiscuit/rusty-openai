use dotenv::dotenv;
use std::env;

use openapi::apis::configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");

    let config = Configuration {
        bearer_access_token: Some(api_key),
        ..Default::default()
    };

    let data = openapi::apis::open_ai_api::list_models(&config).await?;

    println!("Data: {:#?}", data);

    //openai_client::list_models();

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

    // let api_response: ApiResponse = response.json().await?;
    // let completion = api_response.choices.get(0).unwrap().text.trim();

    // println!("Generated text: {}", completion);

    // create_completion_request

    // let request_prompt = create_completion_request
    // // let prompt = CreateCompletionRequestPrompt {
    // //     prompt: Some("Once upon a time...".to_string()),
    // //     ..Default::default()
    // // }
    // let request = CreateCompletionRequest {
    //     prompt: Some("Once upon a time...".to_string()),
    //     // max_tokens: 20,
    //     // temperature: 0.5,
    //     ..Default::default()
    // };
    //open_ai_api::create_completion(configuration, create_completion_request);

    // let mut config = apis::configuration::Configuration::new();
    // config.bearer_access_token = Some("sk-".to_string());

    // let request_prompt = models::CreateCompletionRequestPrompt {
    //     prompt: Some(Some(Box::new("Once upon a time...".to_string()))),
    //     ..Default::default()
    // };

    // apis::open_ai_api::create_completion(
    //     &config,
    //     // apis::configuration::Configuration::new(),
    //     models::CreateCompletionRequest {
    //         prompt: Some(Some(Box::new("Once upon a time...".to_string()))),
    //         ..Default::default()
    //     },
    // );

    Ok(())
}
