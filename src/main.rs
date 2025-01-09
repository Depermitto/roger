use gui::Roger;
use iced::{advanced::graphics::image::image_rs::ImageFormat, window::icon, Application, Settings};

mod dog_api;
mod gui;

fn main() -> iced::Result {
    let icon = icon::from_file_data(
        include_bytes!("../resources/doggo.png"),
        ImageFormat::from_extension("png"),
    )
    .unwrap();
    Roger::run(Settings {
        window: iced::window::Settings {
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    })
}
