use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DogAPI {
    message: Vec<String>,
    status: String
}

impl DogAPI {
    pub fn message(&self) -> &Vec<String> {
        &self.message
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}