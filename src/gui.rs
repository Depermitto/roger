use iced::{widget::{row, column, text, button, Container}, Length, Sandbox, Alignment};

use super::dog_api::{Dog, DogAPI};

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous
}

pub struct Roger {
    dog_history: Vec<Dog>,
    current_index: usize,
    state: State
}

enum State {
    Empty,
    Loaded { dog: Dog },
    Errored
}


impl Roger {
    pub fn rightmost(&self) -> bool {
        self.dog_history.len() == self.current_index
    }

    pub fn leftmost(&self) -> bool {
        self.current_index == 1
    }
}

impl Sandbox for Roger {
    type Message = Message;

    fn new() -> Self {
        Self {
            dog_history: vec![],
            state: State::Empty,
            current_index: 0
        }
    }

    fn title(&self) -> String {
        String::from("Roger")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Previous => {
                self.current_index -= 1;
                self.state = State::Loaded { dog: self.dog_history[self.current_index - 1].clone() };
            }
            Message::Next => match self.rightmost() {
                true => match DogAPI::random_image() {
                    Ok(dog) => {
                        self.state = State::Loaded { dog: dog.clone() };
                        self.dog_history.push(dog);
                        self.current_index += 1;
                    }
                    Err(_) => self.state = State::Errored
                }
                false => {
                    self.current_index += 1;
                    self.state = State::Loaded { dog: self.dog_history[self.current_index - 1].clone() };
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = match &self.state {
            State::Empty => column![
                button("Random dog image").on_press(Message::Next),
            ],
            State::Loaded { dog } => {
                let mut buttons = row![].spacing(10);

                if self.leftmost() || self.dog_history.len() < 2  {
                    buttons = buttons.push(button("Previous dog"));
                } else {
                    buttons = buttons.push(button("Previous dog").on_press(Message::Previous));
                }
                buttons = buttons.push(button("Next dog").on_press(Message::Next));

                column![
                    dog.view(),
                    buttons
                ]
                .spacing(10)
                .padding(10)
                .align_items(Alignment::Center)
            },
            State::Errored => column![
                text("Whoops!, Couldn't find your dog"),
                button("Try again!").on_press(Message::Next)
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