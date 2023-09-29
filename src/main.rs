use json::parse;
use reqwest;
use std::env


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

   let youtube_key =  match env::var("YOUTUBE_API_KEY") {
        Ok(value) => {
            println!("The value of MY_ENV_VARIABLE is: {}", value);
            &value;
        }
        Err(_) => {
            println!("MY_ENV_VARIABLE is not set.");
            "No key found"        
        }
    };


    let result = video_by_channel_id_request("UUpH-VFo4_F2oMYiW8942mQw",youtube_key);
    match result.await {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}


async fn nb_video_request(youtube_key: String) -> Result<String, reqwest::Error> {
    let reponse: String = reqwest::get("https://youtube.googleapis.com/youtube/v3/channels?part=snippet%2CcontentDetails%2Cstatistics&id=UCpH-VFo4_F2oMYiW8942mQw&key=")

        .await?
        .text()
        .await?;

    let data = parse(&reponse).expect("parsing error");
    let nb_video = &data["items"][0]["statistics"]["videoCount"];

    Ok(nb_video.to_string())
}

async fn video_by_channel_id_request(
    playlistId: &str,
    youtube_key: String,
) -> Result<String, reqwest::Error> {
    let reponse: String = reqwest::get("https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2CcontentDetails&playlistId=UUpH-VFo4_F2oMYiW8942mQw&key=")
        .await?
        .text()
        .await?;

    let data = parse(&reponse).expect("parsing error");
    let nb_video = &data["items"]["resourceId"]["videoId"];

    Ok(nb_video.to_string())
}

/*fn parse_json() {
    let json = fs::read_to_string("./yt.json").expect("Error occured while reading fil");

    let data = parse(&json).expect("parser error");

    let count = &data["items"][0]["statistics"]["videoCount"];

    println!("{}", count);
    println!("Hello, world!");
}*/
