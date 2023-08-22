use std::fs;
use json::parse;
use std::collections::HashMap;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = youtube_request();
    match result.await {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }



    Ok(())
}


async fn youtube_request() -> Result<HashMap<String, String>, reqwest::Error> {
    let t = reqwest::get(
        "")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(t)
}

fn parse_json() {
    let json = fs::read_to_string("./yt.json").expect("Error occured while reading fil");

    let data = parse(&json).expect("parser error");

    let count = &data["items"][0];

    println!("{}", count);
    println!("Hello, world!");
}

