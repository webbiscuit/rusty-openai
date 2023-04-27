use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatResponse {
    pub choices: Vec<CreateCompletionResponseChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionResponseChoice {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

pub async fn create_chat(
    api_key: &String,
    request: CreateChatRequest,
) -> Result<CreateChatResponse, Box<dyn std::error::Error>> {
    // println!("Data Request: {:#?}", request);

    // let data_response: serde_json::Value = reqwest::Client::new()
    let data_response: CreateChatResponse = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    // println!("Data Response: {:#?}", data_response);

    Ok(data_response)
}
