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
pub struct ListFilesResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<crate::models::OpenAiFile>,
}

impl ListFilesResponse {
    pub fn new(object: String, data: Vec<crate::models::OpenAiFile>) -> ListFilesResponse {
        ListFilesResponse {
            object,
            data,
        }
    }
}


