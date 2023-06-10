mod dog_api;
mod gui;

use iced::{Settings, Application, window::icon};
use gui::Roger;

fn main() -> iced::Result {
    let icon = icon::from_file("resources/red_dog.ico").unwrap();
    Roger::run(Settings {
        window: iced::window::Settings {
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    })
}