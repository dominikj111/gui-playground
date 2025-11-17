use iced::widget::{button, column, container, text, Column};

use crate::message::{Message, ViewMode};
use crate::styles::button::windows_7_button_style;

pub fn create_menu(current_view: ViewMode) -> Column<'static, Message> {
    let form_button = button(
        container(text("📝 Form").size(16))
            .padding(10)
            .width(iced::Fill)
            .center_x(iced::Fill),
    )
    .width(iced::Fill)
    .style(windows_7_button_style)
    .on_press(Message::ViewChanged(ViewMode::Form));

    let chart_button = button(
        container(text("📊 Chart").size(16))
            .padding(10)
            .width(iced::Fill)
            .center_x(iced::Fill),
    )
    .width(iced::Fill)
    .style(windows_7_button_style)
    .on_press(Message::ViewChanged(ViewMode::Chart));

    let summary_button = button(
        container(text("📋 Summary").size(16))
            .padding(10)
            .width(iced::Fill)
            .center_x(iced::Fill),
    )
    .width(iced::Fill)
    .style(windows_7_button_style)
    .on_press(Message::ViewChanged(ViewMode::Summary));

    let browser_form_button = button(
        container(text("🌐 Browser").size(16))
            .padding(10)
            .width(iced::Fill)
            .center_x(iced::Fill),
    )
    .width(iced::Fill)
    .style(windows_7_button_style)
    .on_press(Message::ViewChanged(ViewMode::BrowserForm));

    let current_view_text = match current_view {
        ViewMode::Form => "Form",
        ViewMode::Chart => "Chart",
        ViewMode::Summary => "Summary",
        ViewMode::BrowserForm => "Browser",
    };

    column![
        container(text("Navigation").size(18))
            .padding(10)
            .width(iced::Fill)
            .center_x(iced::Fill),
        form_button,
        chart_button,
        summary_button,
        browser_form_button,
        container(column![
            text("").size(10),
            text(format!("Current: {}", current_view_text)).size(12),
        ])
        .padding(10)
        .width(iced::Fill)
        .center_x(iced::Fill),
    ]
    .spacing(5)
    .padding(10)
    .width(200)
}
