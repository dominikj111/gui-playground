use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Shadow, Theme};

/// Bootstrap-inspired button style - flat with subtle borders
pub fn bootstrap_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let _palette = _theme.palette();
    
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.48, 0.80))), // Bootstrap primary blue
            border: Border {
                color: Color::from_rgb(0.0, 0.42, 0.70),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.42, 0.70))), // Darker blue on hover
            border: Border {
                color: Color::from_rgb(0.0, 0.36, 0.60),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.36, 0.60))), // Even darker when pressed
            border: Border {
                color: Color::from_rgb(0.0, 0.30, 0.50),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.82, 0.84, 0.86))),
            border: Border {
                color: Color::from_rgb(0.72, 0.74, 0.76),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

/// Secondary button style (gray/outline style)
pub fn bootstrap_secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let _palette = _theme.palette();
    
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.42, 0.46, 0.50))), // Bootstrap secondary gray
            border: Border {
                color: Color::from_rgb(0.36, 0.40, 0.44),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.36, 0.40, 0.44))),
            border: Border {
                color: Color::from_rgb(0.30, 0.34, 0.38),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.30, 0.34, 0.38))),
            border: Border {
                color: Color::from_rgb(0.24, 0.28, 0.32),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.82, 0.84, 0.86))),
            border: Border {
                color: Color::from_rgb(0.72, 0.74, 0.76),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

/// Success button style (green)
pub fn bootstrap_success_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.16, 0.66, 0.36))), // Bootstrap success green
            border: Border {
                color: Color::from_rgb(0.12, 0.58, 0.30),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.12, 0.58, 0.30))),
            border: Border {
                color: Color::from_rgb(0.08, 0.50, 0.24),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.08, 0.50, 0.24))),
            border: Border {
                color: Color::from_rgb(0.04, 0.42, 0.18),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.82, 0.84, 0.86))),
            border: Border {
                color: Color::from_rgb(0.72, 0.74, 0.76),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

/// Danger/warning button style (red)
pub fn bootstrap_danger_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.86, 0.20, 0.27))), // Bootstrap danger red
            border: Border {
                color: Color::from_rgb(0.78, 0.16, 0.23),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.78, 0.16, 0.23))),
            border: Border {
                color: Color::from_rgb(0.70, 0.12, 0.19),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.70, 0.12, 0.19))),
            border: Border {
                color: Color::from_rgb(0.62, 0.08, 0.15),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.82, 0.84, 0.86))),
            border: Border {
                color: Color::from_rgb(0.72, 0.74, 0.76),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

/// Bootstrap-inspired container/card style
pub fn bootstrap_card_style(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(palette.background)),
        border: Border {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.125), // Light border
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: Some(palette.text),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.075),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
        snap: false,
    }
}

/// Form control container style
pub fn form_group_style(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: None,
        border: Border::default(),
        text_color: Some(palette.text),
        shadow: Shadow::default(),
        snap: false,
    }
}

/// Input field style (for text inputs)
pub fn bootstrap_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette();
    
    match status {
        text_input::Status::Active => text_input::Style {
            background: Background::Color(palette.background),
            border: Border {
                color: Color::from_rgb(0.81, 0.83, 0.85), // Bootstrap form-control border
                width: 1.0,
                radius: 4.0.into(),
            },
            icon: palette.text,
            placeholder: Color::from_rgb(0.42, 0.46, 0.50),
            value: palette.text,
            selection: Color::from_rgba(0.0, 0.48, 0.80, 0.3),
        },
        text_input::Status::Hovered => text_input::Style {
            background: Background::Color(palette.background),
            border: Border {
                color: Color::from_rgb(0.52, 0.60, 0.68), // Darker border on hover
                width: 1.0,
                radius: 4.0.into(),
            },
            icon: palette.text,
            placeholder: Color::from_rgb(0.42, 0.46, 0.50),
            value: palette.text,
            selection: Color::from_rgba(0.0, 0.48, 0.80, 0.3),
        },
        text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(palette.background),
            border: Border {
                color: Color::from_rgb(0.0, 0.48, 0.80), // Blue border when focused (Bootstrap focus)
                width: 2.0,
                radius: 4.0.into(),
            },
            icon: palette.text,
            placeholder: Color::from_rgb(0.42, 0.46, 0.50),
            value: palette.text,
            selection: Color::from_rgba(0.0, 0.48, 0.80, 0.3),
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(Color::from_rgb(0.91, 0.92, 0.93)),
            border: Border {
                color: Color::from_rgb(0.81, 0.83, 0.85),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon: Color::from_rgb(0.5, 0.5, 0.5),
            placeholder: Color::from_rgb(0.5, 0.5, 0.5),
            value: Color::from_rgb(0.5, 0.5, 0.5),
            selection: Color::TRANSPARENT,
        },
    }
}
