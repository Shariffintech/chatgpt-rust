use dotenv::dotenv;
use std::env;
use std::process;
use hyper::body::Buf;
use hyper::{header, Client, Body, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize,Serialize};
use spinners::{Spinners, Spinners};
use std::io::{stdin, stdout, Read, Write};


//a struct for the choices
#[derive(Serialize, Deserialize)]
struct OAIChoices {
    choices: Vec<String>,
}


//struct for request to api
//structs to work with the api response

// actix async main function
// load env variables
//create httpconnector, hyper
//create httpclient, hyper
//url to make the request
// prompt for user input to chatgpt
// token, in the header

//loop  - inside the loop to read user input
// spinner wait for response from chatgpt
// request to chatgpt for every single user input
// response and print response


