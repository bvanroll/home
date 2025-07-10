use serde::Deserialize; 

#[derive(Deserialize)]
struct Channel {
    id: String,
    name: String,
    kind: String,
}

#[tokio::main]
async fn main() {
    let channellist = "https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json";
    //getting the list of channels with id's first 
    //
    //https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json
    let resp = reqwest::get(channellist)//.await.unwrap().text().await;
                    .await.unwrap()
                    .text().await.unwrap();
//    let text = match resp {
//        Ok(text) => text,
//        Err(error) => println!("NOPE"),
//    };                                          //
    let channels =serde_json::from_str::<Vec<Channel>>(&resp).unwrap();
    for i in channels {
        println!{"{:#?}", i.name};
    }

}
