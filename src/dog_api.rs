use bytes::Bytes;
use iced::{widget::image, Element, Length};
use serde::Deserialize;

use super::gui::Message;

pub struct DogAPI;

#[derive(Debug, Clone)]
pub struct Dog {
    image: image::Handle,
}

impl DogAPI {
    #[tokio::main]
    pub async fn random_image() -> Result<Dog, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            message: String,
        }

        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await?
            .text()
            .await?;
        let response: Response = serde_json::from_str(&response)?;
        let bytes = reqwest::get(response.message).await?.bytes().await?;

        Ok(Dog::new(bytes))
    }
}

impl Dog {
    pub fn new(bytes: Bytes) -> Self {
        Self {
            image: image::Handle::from_memory(bytes),
        }
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
    JSONParsingError,
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
