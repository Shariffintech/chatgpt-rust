use dotenv::dotenv;
use std::env;
use hyper::body::Buf;
use hyper::{header, body::Body, Request};
use std::string::String;  
use oai_request::OAIRequestdata;
use crate::oai_choices::OAIChoices;
use oai_response::OAIResponse;
mod oai_request;
mod oai_choices;
mod oai_response;

//create a httpsconnector with this
use hyper_tls::HttpsConnector;
// use spinners::{Spinners, Spinner};
use spinner::SpinnerBuilder;
use std::io::{stdin, stdout,  Write};


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
        println!("{}", json.choices()[0].text);
    }
    
    Ok(())
}

