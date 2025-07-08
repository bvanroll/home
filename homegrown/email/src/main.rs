
struct Channel {
    id: String,
    name: String,
    kind: String
}

#[tokio::main]
async fn main() {
    let channellist = "https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.yaml";
    //getting the list of channels with id's first 
    //
    //https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json
    let resp = reqwest::get(channellist)
        .await.
        .text()
        .await;
    let channels: Vec<Channel> =serde_yaml_ng::from_str(&resp);

}
