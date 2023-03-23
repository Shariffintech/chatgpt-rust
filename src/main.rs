use dotenv::dotenv;
use std::env;
use hyper::body::Buf;
use hyper::{header, body::Body, Request};
use std::string::String;  

//create a httpsconnector with this
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize,Serialize};
// use spinners::{Spinners, Spinner};
use spinner::SpinnerBuilder;
use std::io::{stdin, stdout,  Write};

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
    logprob: Option<u8>,
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
async fn main()-> Result<(), Box<dyn std::error::Error + Send + Sync>> {

// load env variables
    dotenv().ok();
    //create httpclient, hyper
    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build(https);
    
    //get the api url
    let api_url = "http://api.openai.com/v1/engines/text-davinci-002/completions";
    let preamble = "What is a lifetime in rust?";
    let oai_token: String = env::var("OPENAI_TOKEN").unwrap();
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
            .expect( "Failed to read line");
            println!("");

             //spinners
            let spin_spin = SpinnerBuilder::new("\t\tOpenAI is Thinking...".into()).start();
            spin_spin.message("Loading...".to_string()).expect( "Failed to load");
       
           

        //send request to api
        let request_data = OAIRequestdata{
            prompt: format!("{}{}", preamble, user_text),
            max_tokens: 600,
            temperature: 1.0,
            frequency: 1.0,
        }; 

        let body = Body::from(serde_json::to_vec(&request_data)?);
        //url to make the request
        let request = Request::post(api_url)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_value)
        .body(body)
        .unwrap();

        // grab the client provided by hyper and send the request to client
        //return response and print it  
        let response = client.request(request).await?;
        let body = hyper::body::aggregate(response).await?;
        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        spin_spin.close();
        println!("");
        println!("{}", json.choices[0].text);
    }
    
    Ok(())
}

