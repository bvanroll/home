use serde::Deserialize; 
use std::env;

#[derive(Deserialize)]
struct Channel {
    id: String,
    name: String,
    kind: String,
}

#[tokio::main]
async fn main() {
    let channellist = "https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json";
    let resp = reqwest::get(channellist)//.await.unwrap().text().await;
                    .await.unwrap()
                    .text().await.unwrap();
    let channels = serde_json::from_str::<Vec<Channel>>(&resp).unwrap();
    let apikey = env::var("APIKEY").unwrap();
    for i in channels {
        let id = i.id;
        let videos_request = format!("https://www.googleapis.com/youtube/v3/channels?id={id}&key={apikey}&part=contentDetails");
        println!("{}", videos_request);
    }

}
