use serde_derive::{Deserialize,Serialize};



//a struct for the choices
#[derive(Serialize, Deserialize)]
pub struct OAIChoices {
    pub text: String,
    index: u8,
    logprob: Option<u8>,
    finish_reason: String,
}

// impl OAIResponse {
//     pub fn choices(&self) -> &Vec<OAIChoices> {
//         &self.choices
//     }
// }
