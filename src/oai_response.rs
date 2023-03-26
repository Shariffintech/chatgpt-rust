use serde_derive::{Deserialize,Serialize};
use crate::oai_choices::OAIChoices;


//struct to work with the api response
#[derive(Serialize, Deserialize)]
pub struct OAIResponse {
    id: Option<String>,
    object: String,
    created:Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}


impl OAIResponse {
    pub fn choices(&self) -> &Vec<OAIChoices> {
        &self.choices
    }
}
