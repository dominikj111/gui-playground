use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Shadow, Theme};

/// Plain HTML-style button - minimal styling like unstyled HTML
pub fn plain_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();
    
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.94, 0.94))), // Light gray like default HTML button
            border: Border {
                color: Color::from_rgb(0.67, 0.67, 0.67), // Gray border
                width: 1.0,
                radius: 2.0.into(), // Minimal rounding
            },
            text_color: palette.text,
            shadow: Shadow::default(), // No shadow
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.88, 0.88, 0.88))), // Slightly darker on hover
            border: Border {
                color: Color::from_rgb(0.60, 0.60, 0.60),
                width: 1.0,
                radius: 2.0.into(),
            },
            text_color: palette.text,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.82, 0.82, 0.82))), // Even darker when pressed
            border: Border {
                color: Color::from_rgb(0.50, 0.50, 0.50),
                width: 1.0,
                radius: 2.0.into(),
            },
            text_color: palette.text,
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.94, 0.94))),
            border: Border {
                color: Color::from_rgb(0.80, 0.80, 0.80),
                width: 1.0,
                radius: 2.0.into(),
            },
            text_color: Color::from_rgb(0.67, 0.67, 0.67),
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

/// Plain HTML input style - like unstyled text input
pub fn plain_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette();
    
    match status {
        text_input::Status::Active => text_input::Style {
            background: Background::Color(Color::WHITE),
            border: Border {
                color: Color::from_rgb(0.67, 0.67, 0.67), // Simple gray border
                width: 1.0,
                radius: 0.0.into(), // No rounding - pure HTML style
            },
            icon: palette.text,
            placeholder: Color::from_rgb(0.67, 0.67, 0.67),
            value: Color::BLACK,
            selection: Color::from_rgba(0.67, 0.84, 1.0, 0.5), // Light blue selection
        },
        text_input::Status::Hovered => text_input::Style {
            background: Background::Color(Color::WHITE),
            border: Border {
                color: Color::from_rgb(0.50, 0.50, 0.50), // Slightly darker on hover
                width: 1.0,
                radius: 0.0.into(),
            },
            icon: palette.text,
            placeholder: Color::from_rgb(0.67, 0.67, 0.67),
            value: Color::BLACK,
            selection: Color::from_rgba(0.67, 0.84, 1.0, 0.5),
        },
        text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(Color::WHITE),
            border: Border {
                color: Color::BLACK, // Black border when focused (like HTML)
                width: 2.0,
                radius: 0.0.into(),
            },
            icon: palette.text,
            placeholder: Color::from_rgb(0.67, 0.67, 0.67),
            value: Color::BLACK,
            selection: Color::from_rgba(0.67, 0.84, 1.0, 0.5),
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(Color::from_rgb(0.94, 0.94, 0.94)),
            border: Border {
                color: Color::from_rgb(0.80, 0.80, 0.80),
                width: 1.0,
                radius: 0.0.into(),
            },
            icon: Color::from_rgb(0.67, 0.67, 0.67),
            placeholder: Color::from_rgb(0.80, 0.80, 0.80),
            value: Color::from_rgb(0.67, 0.67, 0.67),
            selection: Color::TRANSPARENT,
        },
    }
}

/// Plain container - no styling
pub fn plain_container_style(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(palette.background)),
        border: Border::default(), // No border
        text_color: Some(palette.text),
        shadow: Shadow::default(), // No shadow
        snap: false,
    }
}

/// Plain fieldset/form group - with simple border like HTML fieldset
pub fn plain_fieldset_style(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: None,
        border: Border {
            color: Color::from_rgb(0.75, 0.75, 0.75),
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(palette.text),
        shadow: Shadow::default(),
        snap: false,
    }
}
