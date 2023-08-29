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


async fn youtube_request() -> Result<i16, reqwest::Error> {
    let reponse = reqwest::get("https://youtube.googleapis.com/youtube/v3/channels?part=snippet%2CcontentDetails%2Cstatistics&id=UCpH-VFo4_F2oMYiW8942mQw&key=")
        .await?
        .json()
        .await?;

    let data = parse(&reponse).expect("parsing error");
    let nbVideo = &data["items"][0]["statistics"]["videoCount"].as_i16().expect("convert to i16 error");

    Ok(*nbVideo)
}

fn parse_json() {
    let json = fs::read_to_string("./yt.json").expect("Error occured while reading fil");

    let data = parse(&json).expect("parser error");

    let count = &data["items"][0]["statistics"]["videoCount"];

    println!("{}", count);
    println!("Hello, world!");
}

