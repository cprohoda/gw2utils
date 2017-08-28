extern crate hyper;

mod auth;
mod gw2api_request;

// use std::io::write;
// use hyper::Client;
use hyper::header::Headers;
use auth::set_auth;

fn main() {
    let client = Client::new();
    // let response = 
    let mut headers = Headers::new();
    set_auth(&mut headers, "apikey");
}