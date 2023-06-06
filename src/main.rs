mod endpoints;
mod structs {
    pub mod dog_api;
    pub mod window;
}

use endpoints::by_breed;
use structs::dog_api::DogAPI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = by_breed("hound");
    let dog = DogAPI::new(&prompt)
        .await?;

    println!("{:#?}", dog.message()[0]);
    Ok(())
}