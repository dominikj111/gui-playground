use iced::widget::{column, container, text, Column};
use iced::Center;

use crate::app::App;
use crate::message::Message;
use crate::styles::theme::ThemeType;

pub fn create_summary_view(app: &App) -> Column<'_, Message> {
    column![
        text("Summary View").size(32),
        container(
            column![
                text("Application Summary").size(24),
                text(format!(
                    "Name: {}",
                    if app.name.is_empty() {
                        "Not set"
                    } else {
                        &app.name
                    }
                )),
                text(format!(
                    "Email: {}",
                    if app.email.is_empty() {
                        "Not set"
                    } else {
                        &app.email
                    }
                )),
                text(format!("Age: {}", app.age)),
                text(format!("Experience: {:.1} years", app.experience)),
                text(format!(
                    "Role: {}",
                    app.selected_role
                        .map(|r| r.to_string())
                        .unwrap_or_else(|| "Not selected".to_string())
                )),
                text(format!(
                    "Theme: {}",
                    ThemeType::ALL
                        .iter()
                        .find(|t| t.to_theme() == app.selected_theme)
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "Unknown".to_string())
                )),
                text(format!("Counter: {}", app.value)),
            ]
            .spacing(10)
        )
        .padding(20),
        container(
            column![
                text("Counter").size(24),
                text(format!("Current value: {}", app.value)).size(16),
            ]
            .spacing(10)
        )
        .padding(20),
    ]
    .spacing(20)
    .padding(20)
    .align_x(Center)
}
