use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionRequest {
    pub model: String,
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionResponse {
    pub choices: Vec<CreateCompletionResponseChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionResponseChoice {
    pub text: String,
}

pub async fn create_completion(
    api_key: &String,
    request: CreateCompletionRequest,
) -> Result<CreateCompletionResponse> {
    // let data_response: serde_json::Value = reqwest::Client::new()
    let data_response: CreateCompletionResponse = reqwest::Client::new()
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    // println!("Data Response: {:#?}", data_response);

    Ok(data_response)
}
