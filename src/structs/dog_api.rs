use std::collections::HashMap;
use bytes::Bytes;
use iced::{widget::image, Element, Length};
use serde::Deserialize;

use super::gui::Message;

pub struct DogAPI;

#[derive(Debug, Clone)]
pub struct Dog {
    image: image::Handle
}

impl DogAPI {
    #[tokio::main]
    pub async fn random_image() -> Result<Dog, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            message: String,
            status: String
        }

        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await?
            .text()
            .await?;

        // If the previous request succeded we can assume this one will too
        let response: Response = serde_json::from_str(&response).unwrap();

        let bytes = reqwest::get(response.message)
            .await?
            .bytes()
            .await?;

        Ok(Dog::new(bytes))
    }


    pub async fn all_breeds() -> Result<Vec<HashMap<String, Vec<String>>>, Error> {

        #[derive(Deserialize)]
        struct AllBreeds {
            message: Vec<HashMap<String, Vec<String>>>,
            status: String
        }

        let full_body_text_response = reqwest::get(
            "https://dog.ceo/api/breeds/list/all"
        )
        .await?
        .text()
        .await?;

        let all_breeds_container: AllBreeds = serde_json::from_str(&full_body_text_response)?;

        Ok(all_breeds_container.message)
    }
}

impl Dog {
    pub fn new(bytes: Bytes) -> Self {
        let image_handle = image::Handle::from_memory(bytes);

        Self { image: image_handle }
    }

    pub fn view(&self) -> Element<Message> {
        image::Image::new(self.image.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Error {
    APIError,
    JSONParsingError
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        dbg!(error);

        Error::APIError
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        dbg!(error);

        Error::JSONParsingError
    }
}