mod structs {
    pub mod dog_api;
    pub mod gui;
}

use iced::{Settings, Application};
use structs::gui::Roger;

fn main() -> iced::Result {
    Roger::run(Settings::default())
}