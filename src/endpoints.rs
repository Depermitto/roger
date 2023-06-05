use lazy_static::lazy_static;

lazy_static! {
    pub static ref breed_list: String = String::from("https://dog.ceo/api/breeds/list/all");
}

pub fn by_breed(breed: &str) -> String {
    format!("https://dog.ceo/api/breed/{}/images", breed)
}