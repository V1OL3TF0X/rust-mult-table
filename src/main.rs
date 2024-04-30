use iced::{window, Application, Settings};
use image::GenericImageView;
use rust_mult_table::app::MultiplicationTableApp;
fn main() -> iced::Result {
    std::env::set_var("RUST_BACKTRACE", "1");
    let icon = image::open(format!(
        "{}\\src\\assets\\playstore.png",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    let (x, y) = icon.dimensions();

    MultiplicationTableApp::run(Settings {
        window: window::Settings {
            min_size: Some((1024, 550)),
            icon: window::icon::from_rgba(icon.as_bytes().to_owned(), x, y).ok(),
            ..Default::default()
        },
        ..Default::default()
    })
}
