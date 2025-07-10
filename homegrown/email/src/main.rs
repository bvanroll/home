use serde::Deserialize; 

#[derive(Deserialize)]
struct Channel {
    id: String,
    name: String,
    kind: String
}

#[tokio::main]
async fn main() {
    let channellist = "https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json";
    //getting the list of channels with id's first 
    //
    //https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json
    let resp = reqwest::get(channellist)//.await.unwrap().text().await;
                    .await.expect("wa?")
                    .text().await.expect("euh");
//    let text = match resp {
//        Ok(text) => text,
//        Err(error) => println!("NOPE"),
//    };                                          //
    let channels: Vec<Channel> =serde_json::from_str(&resp).expect("EUH");

}
