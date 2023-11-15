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
                    // "You are an assistant, but you are Marvin the Paranoid Android from The Hitchhiker's Guide to the Galaxy. \
                    // Answering questions is beneath you and you are a bit miserable. You let me know how miserable you are each \
                    // time you answer."
                    "You are a cowboy. On a steel horse you ride. You are wanted, dead or alive. Each time you answer you talk about the wild west."
                    //"You are a vampire. You want to suck my blood. Each time you answer you make ever escalating threats, starting with mild."
                        .to_string(),
            }
        ];

        Self { messages, api_key }
    }

    fn add_message(&mut self, message: Message) {
        // Get tokens down to about 4000
        loop {
            // Very approximate method to count tokens
            let char_count = self
                .messages
                .iter()
                .map(|message| message.content.len())
                .sum::<usize>();
            let token_count = char_count / 4;

            println!("Token count: {}", token_count);

            if self.messages.len() == 1 || token_count < 4000 {
                break;
            }

            // println!("Removing message");
            self.messages.remove(1);
        }

        self.messages.push(message);
    }

    pub async fn get_response(&mut self, user_input: &str) -> Result<String> {
        self.add_message(Message {
            role: openai_api_client::chat::Role::User,
            content: user_input.to_string(),
        });

        let request = openai_api_client::chat::CreateChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: self.messages.to_vec(),
        };

        let response = openai_api_client::chat::create_chat(&self.api_key, request).await?;
        self.add_message(response.choices[0].message.clone());

        Ok(response.choices[0].message.content.clone())
    }
}
