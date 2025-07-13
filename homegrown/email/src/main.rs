use serde::Deserialize; 
use std::env;
use chrono;
use dotenv::dotenv;

#[derive(Deserialize)]
struct Channel {
    id: String,
    name: String,
    kind: String,
}

struct Videos {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() {
    dotenv.ok();
    let channellist = "https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json";
    let resp = reqwest::get(channellist)//.await.unwrap().text().await;
                    .await.unwrap()
                    .text().await.unwrap();
    let channels = serde_json::from_str::<Vec<Channel>>(&resp).unwrap();
    let apikey = env::var("APIKEY").unwrap();
//    let lastMonday = 
    for i in channels {
        let id = i.id;
        
        let videos_request = format!("https://www.googleapis.com/youtube/v3/search/?channelId={id}&part=snippet,id&order=date&publishedAfter={date}&key={apikey}");
        //let videos_request = format!("https://www.googleapis.com/youtube/v3/channels?id={id}&key={apikey}&part=contentDetails");
        println!("{}", videos_request);
    }

}
