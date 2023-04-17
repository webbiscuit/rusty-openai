/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateModerationRequest {
    #[serde(rename = "input")]
    pub input: Box<crate::models::CreateModerationRequestInput>,
    /// Two content moderations models are available: `text-moderation-stable` and `text-moderation-latest`.  The default is `text-moderation-latest` which will be automatically upgraded over time. This ensures you are always using our most accurate model. If you use `text-moderation-stable`, we will provide advanced notice before updating the model. Accuracy of `text-moderation-stable` may be slightly lower than for `text-moderation-latest`. 
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl CreateModerationRequest {
    pub fn new(input: crate::models::CreateModerationRequestInput) -> CreateModerationRequest {
        CreateModerationRequest {
            input: Box::new(input),
            model: None,
        }
    }
}


