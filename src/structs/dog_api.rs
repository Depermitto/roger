use std::error::Error;
use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DogAPI {
    message: Vec<String>,
    status: String
}

impl DogAPI {
    pub async fn new(prompt: &str) -> Result<Self, Box<dyn Error>> {
        let text = Self::response(prompt)
            .await?
            .text()
            .await?;
        let text: Self = serde_json::from_str(&text)?;
        Ok(text)
    }

    pub async fn new_prompt(&mut self, prompt: &str) -> Result<(), Box<dyn Error>> {
        let dog = Self::new(prompt).await?;
        self.message = dog.message;
        self.status = dog.status;
        Ok(())
    }

    async fn response(prompt: &str) -> Result<Response, Box<dyn Error>> {
        let response = reqwest::get(prompt)
            .await?;

        Ok(response)
    }

    pub fn message(&self) -> Vec<String> {
        self.message.clone()
    }

    pub fn status(&self) -> String {
        self.status.clone()
    }
}