// Module declarations
mod app;
mod message;
mod styles;
mod components;
mod views;
mod widgets;
mod utils;

use app::App;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}
