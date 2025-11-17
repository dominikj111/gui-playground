use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 200, "Hello, World!");

    let mut frame = Frame::new(100, 50, 200, 40, "Hello, World!");
    let mut btn = Button::new(150, 120, 100, 40, "Change Text");

    btn.set_callback(move |_| {
        frame.set_label("Text Changed!");
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}

