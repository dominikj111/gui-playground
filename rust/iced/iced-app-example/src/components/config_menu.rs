use iced::widget::{button, column, container, mouse_area, text, Column};
use iced::{Background, Border, Color, Element, Shadow};
use std::time::Instant;

use crate::message::Message;
use crate::utils::animation::get_animation_progress;

pub fn create_config_menu(
    is_open: bool,
    animation_start: Option<Instant>,
) -> Element<'static, Message> {
    // Menu content with rounded corners and limited height
    let menu_content = column![
        container(
            text("⚙️ Configuration")
                .size(20)
                .width(iced::Length::Fill) // Prevent wrapping
                .wrapping(iced::widget::text::Wrapping::None) // No text wrapping
        )
        .padding(15)
        .width(250) // Fixed width to prevent shrinking
        .style(|_theme| {
            container::Style {
                background: None,
                text_color: Some(Color::WHITE),
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            }
        }),
        
        container(
            column![
                text("Application Settings")
                    .size(14)
                    .wrapping(iced::widget::text::Wrapping::None),
                container(text("")).height(10),
                
                button(
                    text("Reset All Data")
                        .wrapping(iced::widget::text::Wrapping::None)
                )
                    .on_press(Message::ConfigResetData)
                    .padding(10)
                    .width(iced::Fill),
                    
                container(text("")).height(5),
                
                button(
                    text("Export Settings")
                        .wrapping(iced::widget::text::Wrapping::None)
                )
                    .on_press(Message::ConfigExportSettings)
                    .padding(10)
                    .width(iced::Fill),
                    
                container(text("")).height(5),
                
                button(
                    text("Import Settings")
                        .wrapping(iced::widget::text::Wrapping::None)
                )
                    .on_press(Message::ConfigImportSettings)
                    .padding(10)
                    .width(iced::Fill),
                    
                container(text("")).height(20),
                
                text("About")
                    .size(14)
                    .wrapping(iced::widget::text::Wrapping::None),
                container(text("")).height(5),
                text("iced Demo App v1.0")
                    .size(12)
                    .wrapping(iced::widget::text::Wrapping::None),
                text("Built with iced")
                    .size(10)
                    .wrapping(iced::widget::text::Wrapping::None),
            ]
            .spacing(5)
        )
        .padding(15)
        .width(250) // Fixed width to prevent shrinking
        .style(|_theme| {
            container::Style {
                background: None,
                text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            }
        }),
    ]
    .width(250);
    
    let menu_column = column![
        container(menu_content)
            .style(move |_theme| {
                container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 0.75))),
                    text_color: Some(Color::WHITE),
                    border: Border {
                        color: Color::from_rgba(0.3, 0.3, 0.3, 0.6),
                        width: 1.0,
                        radius: 8.0.into(), // Rounded corners
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                        offset: iced::Vector::new(4.0, 0.0),
                        blur_radius: 10.0,
                    },
                    snap: false,
                }
            })
            .width(250)
            .height(500) // Limited height, not full screen
    ];
    
    // Wrap with mouse_area to capture all mouse events and prevent click-through
    mouse_area(menu_column)
        .on_press(Message::ConfigMenuInteraction)
        .on_enter(Message::ConfigMenuInteraction)
        .on_move(|_| Message::ConfigMenuInteraction)
        .into()
}

pub fn get_menu_offset(is_open: bool, animation_start: Option<Instant>) -> f32 {
    let progress = get_animation_progress(animation_start, 300.0);
    
    if is_open {
        -250.0 * (1.0 - progress) // Slide in from left
    } else {
        if animation_start.is_some() {
            -250.0 * progress // Slide out to left
        } else {
            -250.0 // Hidden by default
        }
    }
}
