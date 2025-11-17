use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text, text_input, Column};
use iced::Theme;

use crate::app::App;
use crate::message::{BrowserOption, Message};
use crate::styles::plain::{plain_button_style, plain_container_style, plain_fieldset_style, plain_input_style};
use crate::styles::web::{
    bootstrap_button_style, bootstrap_card_style, bootstrap_danger_button_style,
    bootstrap_input_style, bootstrap_secondary_button_style, bootstrap_success_button_style,
    form_group_style,
};

pub fn create_browser_form_view(app: &App) -> Column<'_, Message> {
    column![
        // Header
        text("Plain HTML Style Form").size(32),
        text("Demonstrating unstyled HTML appearance with iced").size(14),
        
        // Form Card
        container(
            column![
                // Text Input Group
                container(
                    column![
                        text("Text Input:"),
                        text_input("Enter text...", &app.browser_text)
                            .on_input(Message::BrowserTextChanged)
                            .padding(5)
                            .style(plain_input_style),
                    ]
                    .spacing(5)
                )
                .style(plain_fieldset_style)
                .padding(10),
                // Password Input Group
                container(
                    column![
                        text("Password:"),
                        text_input("Enter password...", &app.browser_password)
                            .on_input(Message::BrowserPasswordChanged)
                            .secure(true)
                            .padding(5)
                            .style(plain_input_style),
                    ]
                    .spacing(5)
                )
                .style(plain_fieldset_style)
                .padding(10),
                // Slider Group
                container(
                    column![
                        text(format!("Range: {:.0}", app.browser_slider_value)),
                        slider(0.0..=100.0, app.browser_slider_value, Message::BrowserSliderChanged),
                    ]
                    .spacing(5)
                )
                .style(plain_fieldset_style)
                .padding(10),
                // Checkbox Group
                container(
                    column![
                        checkbox("Enable feature", app.browser_checkbox)
                            .on_toggle(Message::BrowserCheckboxToggled),
                    ]
                    .spacing(5)
                )
                .style(plain_fieldset_style)
                .padding(10),
                // Dropdown Group
                container(
                    column![
                        text("Select:"),
                        pick_list(
                            &BrowserOption::ALL[..],
                            app.browser_selected_option,
                            Message::BrowserOptionSelected
                        )
                        .placeholder("Choose an option...")
                        .width(250),
                    ]
                    .spacing(5)
                )
                .style(plain_fieldset_style)
                .padding(10),
                // Buttons
                container(
                    row![
                        button("Submit")
                            .on_press(Message::BrowserFormSubmit)
                            .padding(8)
                            .style(plain_button_style),
                        button("Reset")
                            .on_press(Message::BrowserFormSubmit)
                            .padding(8)
                            .style(plain_button_style),
                        button("Cancel")
                            .padding(8)
                            .style(plain_button_style),
                    ]
                    .spacing(5)
                )
                .padding(10),
            ]
            .spacing(5)
        )
        .style(plain_container_style)
        .padding(10),
        // Footer note
        text("This view demonstrates plain HTML-style controls (no CSS), while other views use custom styling")
            .size(12),
    ]
    .spacing(10)
    .padding(20)
}
