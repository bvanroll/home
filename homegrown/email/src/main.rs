use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //getting the list of channels with id's first 
    let resp = reqwest::get("https://raw.githubusercontent.com/bvanroll/home/static/yters.json")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())

}
