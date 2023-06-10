#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/red_dog_portrait.ico");
    res.compile().unwrap();
}