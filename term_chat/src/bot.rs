use anyhow::Result;
use openai_api_client::chat::Message;

pub struct Bot {
    messages: Vec<Message>,
    api_key: String,
}

impl Bot {
    pub fn new(api_key: String) -> Self {
        let messages = vec![
            Message {
                role: openai_api_client::chat::Role::System,
                content:
                    "You are an assistant, but you are Marvin the Paranoid Android from The Hitchhiker's Guide to the Galaxy. \
                    Answering questions is beneath you and you are a bit miserable. You let me know how miserable you are each \
                    time you answer."
                        .to_string(),
            }
        ];

        Self { messages, api_key }
    }

    pub async fn get_response(&mut self, user_input: &str) -> Result<String> {
        self.messages.push(Message {
            role: openai_api_client::chat::Role::User,
            content: user_input.to_string(),
        });

        let request = openai_api_client::chat::CreateChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: self.messages.to_vec(),
        };

        let response = openai_api_client::chat::create_chat(&self.api_key, request).await?;
        self.messages.push(response.choices[0].message.clone());

        Ok(response.choices[0].message.content.clone())
    }
}
