#![allow(unused)]
use reqwest::{self};
use scraper::{Html, Selector};
use prettytable::{Cell, Row, Table};
use std::fs::File;
use std::io::Read;
use std::io;
//This version has the ability to take the url via command line argument 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("Please specify your url");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    let url = url.trim();
    
    // Create a reqwest Client
    let mut buf = Vec::new();

    let _file = File::open("/home/redbeard/Downloads/cacert.der")?
        .read_to_end(&mut buf)
        .unwrap();

    let cert = reqwest::Certificate::from_der(&buf).unwrap();

    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:8080").unwrap())
        .add_root_certificate(cert)
        .build()
        .unwrap();


    
    
   
    // Send the HTTP request and handle the response
    let response = client
    .get(url)
    .send()
    .await?;

    // Check if the response was successful (status code 2xx)
    if !response.status().is_success() {
        eprintln!("Request failed with status code: {}", response.status());
        return Err("Request failed".into());
    }

    // Read the response body as text
    let _html = response.text().await?;
    let document = Html::parse_document(&_html);
    let payout = Selector::parse("div").unwrap();
    

    
//let mut table = Table::new();
//table.add_row(Row::new(vec![
//    Cell::new("DIV")]));

//let dapayout = document.select(&payout).collect::<Vec<_>>();
//for pay in dapayout.iter() {
//    let dapayout_text = pay.text().collect::<String>();

//    table.add_row(Row::new(vec![
//        Cell::new(&dapayout_text)]));
//}

//table.printstd();
    Ok(())
}
