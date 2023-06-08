use iced::{widget::{row, column, text, button, Container}, Length, Sandbox, Alignment};

use super::dog_api::{Dog, DogAPI};

#[derive(Debug, Clone)]
pub enum Message {
    Search
}

#[derive(Debug)]
pub enum Roger {
    Empty,
    Loaded { dog: Dog },
    Errored
}


impl Sandbox for Roger {
    type Message = Message;

    fn new() -> Self {
        Self::Empty
    }

    fn title(&self) -> String {
        String::from("Roger")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Search => match DogAPI::random_image() {
                Ok(dog) => *self = Self::Loaded { dog: dog },
                Err(_) => *self = Self::Errored
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = match self {
            Self::Empty => column![
                button("Random dog image").on_press(Message::Search)
            ],
            Self::Loaded { dog } => column![
                dog.view(),
                button("Next dog").on_press(Message::Search)
            ]
            .align_items(Alignment::Center),
            Self::Errored => column![
                text("Whoops!, Couldn't find your dog"),
                button("Try again!").on_press(Message::Search)
            ]
            .align_items(Alignment::Center)
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}