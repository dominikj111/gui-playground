use iced::widget::{button, column, row, text, Column};
use iced::Center;
use std::time::Instant;

use crate::message::Message;
use crate::styles::button::{apply_opacity_to_button_style, windows_7_button_style, windows_7_button_style_with_gray};
use crate::utils::animation::{AnimationDirection, get_animation_progress, get_opacity};

pub fn create_animated_buttons(
    second_button_visible: bool,
    button_animation_start: Option<Instant>,
    button_animation_direction: AnimationDirection,
) -> Column<'static, Message> {
    let is_animating = button_animation_start.is_some();
    let progress = get_animation_progress(button_animation_start, 300.0);
    let opacity = get_opacity(button_animation_direction, progress);
    let first_button_enabled = !second_button_visible && !is_animating;

    let mut items = vec![];

    // Title
    items.push(text("Animated Buttons Demo").size(20).into());

    // Create button row
    let mut button_row_items = vec![];

    // First button - shows second button when clicked
    // Calculate gray amount for smooth disable animation
    let gray_amount = if first_button_enabled {
        0.0
    } else if is_animating && button_animation_direction == AnimationDirection::FadeIn {
        progress
    } else if is_animating && button_animation_direction == AnimationDirection::FadeOut {
        1.0 - progress
    } else {
        1.0
    };

    let first_btn = if first_button_enabled {
        button("Show Button")
            .on_press(Message::ShowSecondButton)
            .style(move |theme, status| {
                windows_7_button_style_with_gray(theme, status, gray_amount)
            })
    } else {
        button("Show Button").style(move |theme, status| {
            windows_7_button_style_with_gray(theme, status, gray_amount)
        })
    };
    button_row_items.push(first_btn.into());

    // Second button - hides itself when clicked (with fade animation)
    if second_button_visible || is_animating {
        let second_btn = button("Hide Me")
            .on_press(Message::HideSecondButton)
            .style(move |theme, status| {
                let style = windows_7_button_style(theme, status);
                apply_opacity_to_button_style(style, opacity)
            });
        button_row_items.push(second_btn.into());
    }

    items.push(row(button_row_items).spacing(10).align_y(Center).into());

    column(items).spacing(10)
}
