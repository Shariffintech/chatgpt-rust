use dotenv::dotenv;
use std::env;
use std::process;
use hyper::body::Buf;
use hyper::{header, Client, Body, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize,Serialize};
use spinners::{Spinners, Spinners};
use std::io::{stdin, stdout, Read, Write};

//struct to work with the api response
#[derive(Serialize, Deserialize)]
struct OAIResponse {
    id: Option<String>,
    object: String,
    created:Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}


//a struct for the choices
#[derive(Serialize, Deserialize)]
struct OAIChoices {
    text: String,
    index: u8,
    logprob: option<u8>,
    finish_reason: String,
}


//struct for request to api
#[derive(Serialize, Deserialize)]
struct OAIRequestdata {
    prompt: String,
    //number of words to generate, not actual tokens
    max_tokens: u16,
    temperature: f32,
    frequency: f32,
}

// tokio async main function
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error + send + Sync>> {
    dotenv.ok();
    //create httpconnector, hyper
    let https = HttpsConnector::new(4)?;
    let client = Client::builder().build(https);
    
    //get the api url
    let api_url = "http://api.openai.com/v1/engines/text-davinci-002/completions";
    let preamble = "What is a lifetime in rust?";
    let oai_token = String = env::var("OPENAI_TOKEN").unwrap();
    let auth_header_value = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    loop{
        //prompt user for input
        println!("<>");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        //read user input
        stdin()
            .read_line(&mut user_text)
            .expect( "Failed to read line")?;
            println!("");

             //spinners
            let spin_spin = Spinners::new(Spinners::Chars("|/-\\"));
            spin_spin.set_message("Loading...");
            spin_spin.start();
           

        //send request to api
        let request_data = OAIRequestdata{
            prompt: format!("{}{}", preamble, user_text),
            max_tokens: 600,
            temperature: 1.0,
            frequency: 1.0,
        } 


          //url to make the request
          api_request.headers_mut().set(header::CONTENT_TYPE, "application/json");
          let mut api_request = Request::new(Body::empty());
    }



  
    
    
    
    
 
    
}
// prompt for user input to chatgpt
// load env variables
//create httpclient, hyper
// token, in the header

//loop  - inside the loop to read user input
// spinner wait for response from chatgpt
// request to chatgpt for every single user input
// response and print response


