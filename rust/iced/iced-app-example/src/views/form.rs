use iced::widget::{button, column, container, pick_list, row, slider, text, text_input, Column};
use iced::Center;

use crate::app::App;
use crate::components::animated_buttons;
use crate::message::{Message, Role};
use crate::styles::theme::ThemeType;

pub fn create_form_view(app: &App) -> Column<'_, Message> {
    column![
        text("Rich Form Application").size(32),
        // Counter section
        container(
            column![
                text("Counter").size(20),
                row![
                    button("Decrement").on_press(Message::Decrement),
                    text(app.value).size(30),
                    button("Increment").on_press(Message::Increment),
                ]
                .spacing(10)
                .align_y(Center)
            ]
            .spacing(10)
        )
        .padding(10),
        // Text inputs section
        container(
            column![
                text("Personal Information").size(20),
                text("Name:").size(14),
                text_input("Enter your name...", &app.name)
                    .on_input(Message::NameChanged)
                    .padding(10),
                text("Email:").size(14),
                text_input("Enter your email...", &app.email)
                    .on_input(Message::EmailChanged)
                    .padding(10),
            ]
            .spacing(5)
        )
        .padding(10),
        // Sliders section
        container(
            column![
                text("Settings").size(20),
                text(format!("Age: {}", app.age)).size(14),
                slider(18..=100, app.age, Message::AgeChanged),
                text(format!("Experience: {:.1} years", app.experience)).size(14),
                slider(0.0..=20.0, app.experience, Message::ExperienceChanged).step(0.5),
            ]
            .spacing(10)
        )
        .padding(10),
        // Dropdowns section
        container(
            column![
                text("Preferences").size(20),
                text("Role:").size(14),
                pick_list(&Role::ALL[..], app.selected_role, Message::RoleSelected)
                    .placeholder("Choose a role...")
                    .width(250),
                text("Theme:").size(14),
                pick_list(
                    &ThemeType::ALL[..],
                    ThemeType::ALL
                        .iter()
                        .find(|t| t.to_theme() == app.selected_theme)
                        .copied(),
                    Message::ThemeSelected
                )
                .placeholder("Choose a theme...")
                .width(250),
            ]
            .spacing(10)
        )
        .padding(10),
        // Animated buttons section
        container(animated_buttons::create_animated_buttons(
            app.second_button_visible,
            app.button_animation_start,
            app.button_animation_direction,
        ))
        .padding(10),
        // Bar Chart section
        container(column![text("Data Visualization").size(20), app.chart.view(),].spacing(10))
            .padding(10),
    ]
    .spacing(20)
    .padding(20)
}
