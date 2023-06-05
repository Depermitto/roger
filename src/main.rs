mod endpoints;
mod structs;

use endpoints::by_breed;
use structs::DogAPI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = by_breed("hound");
    let resp = reqwest::get(prompt)
        .await?
        .text()
        .await?;
    let text: DogAPI = serde_json::from_str(&resp)?;

    println!("{:#?}", text.message()[0]);
    Ok(())
}