use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //getting the list of channels with id's first 
    //https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json
    let resp = reqwest::get("https://raw.githubusercontent.com/bvanroll/home/refs/heads/master/static/yters.json")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
