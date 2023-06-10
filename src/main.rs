mod dog_api;
mod gui;

fn main() -> iced::Result {
    use iced::{Settings, Application, window::icon};
    use gui::Roger;

    let icon_data = include_bytes!("../resources/red_dog_portrait.ico");
    let icon = icon::from_file_data(icon_data, None).unwrap();
    Roger::run(Settings {
        window: iced::window::Settings {
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    })
}