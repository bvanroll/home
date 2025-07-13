use serde::Deserialize; 
use std::env;
use chrono::prelude::*;
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
    dotenv().ok();
    let channellist = "https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json";
    let resp = reqwest::get(channellist)//.await.unwrap().text().await;
                    .await.unwrap()
                    .text().await.unwrap();
    let channels = serde_json::from_str::<Vec<Channel>>(&resp).unwrap();
    let apikey = env::var("APIKEY").unwrap();
//    let lastMonday = 
    let datetime: DateTime<Utc> = chrono::prelude::Utc::now() - chrono::TimeDelta::try_days(7).unwrap() ;
    //let datestr = datetime.
    let date= datetime.format("%Y-%m-%dT%H:%M:%SZ"); //(1970-01-01T00:00:00Z).
    let part="snippet";
    let order = "date";
    for i in channels {
        let id = i.id;
        let videos_request = format!("https://www.googleapis.com/youtube/v3/search/?channelId={id}&part={part},id&order={order}&publishedAfter={date}&key={apikey}");
        //let videos_request = format!("https://www.googleapis.com/youtube/v3/channels?id={id}&key={apikey}&part=contentDetails");
        let resp = reqwest::get(videos_request).await.unwrap().text().await.unwrap();
        println!("{}", resp);
    }

}
