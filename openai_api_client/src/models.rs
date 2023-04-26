use reqwest::Client;
use std::{collections::HashMap, error};

// These require the `serde` dependency.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    created: i32,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    data: Vec<Model>,
}

pub async fn list_models(api_key: &String) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    // let data_response: serde_json::Value = reqwest::Client::new()
    let data_response: Response = reqwest::Client::new()
        .get("https://api.openai.com/v1/models")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?
        .json()
        .await?;

    println!("Data Response: {:#?}", data_response);

    Ok(data_response.data)
}
