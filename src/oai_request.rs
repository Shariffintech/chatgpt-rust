use serde_derive::{Deserialize,Serialize};


//struct for request to api
#[derive(Serialize, Deserialize)]
pub struct OAIRequestdata {
    pub prompt: String,
    //number of words to generate, not actual tokens
    pub max_tokens: u16,
    pub temperature: f32,
    pub frequency: f32,
}
