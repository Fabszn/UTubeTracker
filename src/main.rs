use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    #[derive(Debug, Deserialize, Serialize)]
    struct Video {
        #[serde(default = "path")]
        id: String,
        #[serde(default = "person_info.data.name")]
        title: String,
    }

    fn path() -> String {
        "".to_string()
    }

    let env_ytube_key = env::var("YOUTUBE_API_KEY");

    let youtube_key = match env_ytube_key {
        Ok(value) => {
            println!("env var : YOUTUBE_API_KEY > {}", value);
            value
        }
        Err(_) => {
            println!("YOUTUBE_API_KEY is not set.");
            "No key found".to_string()
        }
    };

    let result = video_by_channel_id_request("UUpH-VFo4_F2oMYiW8942mQw", &youtube_key);
    match result.await {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    };

    Ok(())
}

async fn nb_video_request(playlist_id: &str, youtube_key: &str) -> Result<String, reqwest::Error> {
    let reponse: String = reqwest::get(format!("https://youtube.googleapis.com/youtube/v3/channels?part=snippet%2CcontentDetails%2Cstatistics&id={}&key={}",playlist_id,youtube_key))

        .await?
        .text()
        .await?;

    let data = parse(&reponse).expect("parsing error");
    let nb_video = &data["items"][0]["statistics"]["videoCount"];

    Ok(nb_video.to_string())
}

async fn video_by_channel_id_request(
    playlist_id: &str,
    youtube_key: &str,
) -> Result<String, reqwest::Error> {
    let formatted_url = format!("https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2CcontentDetails&playlistId={}&key={}",   playlist_id,youtube_key);

    println!("{}", formatted_url);
    let reponse: String = reqwest::get(formatted_url).await?.text().await?;

    let data = parse(&reponse).expect("parsing error");
    println!("data from request : {}", &data);
    let nb_video = &data["items"];

    Ok(nb_video.to_string())
}

/*fn parse_json() {
    let json = fs::read_to_string("./yt.json").expect("Error occured while reading fil");

    let data = parse(&json).expect("parser error");

    let count = &data["items"][0]["statistics"]["videoCount"];

    println!("{}", count);
    println!("Hello, world!");
}*/
