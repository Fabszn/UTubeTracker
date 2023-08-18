use std::fs;
use json::parse;
use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //AIzaSyDuRlgSAqroomabL3bMGMdzd-aRdKIV9TM

    

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp);
    parse_json();
    Ok(())
}


fn parse_json() {
    println!("{:#?}","parseJdon");
    let json = fs::read_to_string("./yt.json").expect("Error occured while reading fil");

    let data = parse(&json).expect("parser error");

    let count = &data["items"][0];

    println!("{}", count);
    println!("Hello, world!");
}

