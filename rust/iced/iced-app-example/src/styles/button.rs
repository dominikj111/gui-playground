use iced::widget::button;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

/// Windows 7 style button with animated graying effect
pub fn windows_7_button_style_with_gray(
    theme: &Theme,
    status: button::Status,
    gray_amount: f32,
) -> button::Style {
    let _palette = theme.palette();

    // Interpolate between normal and disabled colors based on gray_amount
    let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;

    match status {
        button::Status::Active => {
            let normal_top = Color::from_rgb(0.95, 0.96, 0.98);
            let normal_mid = Color::from_rgb(0.88, 0.91, 0.95);
            let normal_bot = Color::from_rgb(0.82, 0.86, 0.92);
            let gray_top = Color::from_rgb(0.92, 0.92, 0.92);
            let gray_bot = Color::from_rgb(0.85, 0.85, 0.85);

            button::Style {
                background: Some(Background::Gradient(iced::Gradient::Linear(
                    iced::gradient::Linear::new(iced::Radians(1.5708))
                        .add_stop(
                            0.0,
                            Color::from_rgb(
                                lerp(normal_top.r, gray_top.r, gray_amount),
                                lerp(normal_top.g, gray_top.g, gray_amount),
                                lerp(normal_top.b, gray_top.b, gray_amount),
                            ),
                        )
                        .add_stop(
                            0.5,
                            Color::from_rgb(
                                lerp(normal_mid.r, (gray_top.r + gray_bot.r) / 2.0, gray_amount),
                                lerp(normal_mid.g, (gray_top.g + gray_bot.g) / 2.0, gray_amount),
                                lerp(normal_mid.b, (gray_top.b + gray_bot.b) / 2.0, gray_amount),
                            ),
                        )
                        .add_stop(
                            1.0,
                            Color::from_rgb(
                                lerp(normal_bot.r, gray_bot.r, gray_amount),
                                lerp(normal_bot.g, gray_bot.g, gray_amount),
                                lerp(normal_bot.b, gray_bot.b, gray_amount),
                            ),
                        ),
                ))),
                border: Border {
                    color: Color::from_rgb(
                        lerp(0.44, 0.65, gray_amount),
                        lerp(0.50, 0.65, gray_amount),
                        lerp(0.60, 0.65, gray_amount),
                    ),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Color::from_rgb(
                    lerp(0.1, 0.6, gray_amount),
                    lerp(0.1, 0.6, gray_amount),
                    lerp(0.1, 0.6, gray_amount),
                ),
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.15 * (1.0 - gray_amount)),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
                snap: false,
            }
        }
        _ => windows_7_button_style(theme, status),
    }
}

/// Standard Windows 7 style button
pub fn windows_7_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let _palette = theme.palette();

    match status {
        button::Status::Active => button::Style {
            // Windows 7 uses a subtle gradient - we'll use a light blue-gray
            background: Some(Background::Gradient(iced::Gradient::Linear(
                iced::gradient::Linear::new(iced::Radians(1.5708)) // 90 degrees (top to bottom)
                    .add_stop(0.0, Color::from_rgb(0.95, 0.96, 0.98)) // Light top
                    .add_stop(0.5, Color::from_rgb(0.88, 0.91, 0.95)) // Mid gradient
                    .add_stop(1.0, Color::from_rgb(0.82, 0.86, 0.92)), // Darker bottom
            ))),
            border: Border {
                color: Color::from_rgb(0.44, 0.50, 0.60), // Subtle gray-blue border
                width: 1.0,
                radius: 4.0.into(), // Rounded corners like Windows 7
            },
            text_color: Color::from_rgb(0.1, 0.1, 0.1), // Dark gray text
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            snap: false,
        },
        button::Status::Hovered => button::Style {
            // Brighter gradient on hover - Windows 7 signature light blue glow
            background: Some(Background::Gradient(iced::Gradient::Linear(
                iced::gradient::Linear::new(iced::Radians(1.5708))
                    .add_stop(0.0, Color::from_rgb(0.92, 0.95, 0.99)) // Lighter blue-white
                    .add_stop(0.5, Color::from_rgb(0.82, 0.90, 0.98)) // Light blue
                    .add_stop(1.0, Color::from_rgb(0.70, 0.84, 0.97)), // Windows 7 blue
            ))),
            border: Border {
                color: Color::from_rgb(0.26, 0.47, 0.69), // Stronger blue border on hover
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::from_rgb(0.05, 0.05, 0.05),
            shadow: Shadow {
                color: Color::from_rgba(0.26, 0.47, 0.69, 0.4), // Blue glow
                offset: Vector::new(0.0, 0.0),
                blur_radius: 6.0,
            },
            snap: false,
        },
        button::Status::Pressed => button::Style {
            // Darker, inverted gradient when pressed
            background: Some(Background::Gradient(iced::Gradient::Linear(
                iced::gradient::Linear::new(iced::Radians(1.5708))
                    .add_stop(0.0, Color::from_rgb(0.70, 0.78, 0.88)) // Darker top
                    .add_stop(0.5, Color::from_rgb(0.76, 0.83, 0.91)) // Mid
                    .add_stop(1.0, Color::from_rgb(0.82, 0.88, 0.94)), // Lighter bottom (inverted)
            ))),
            border: Border {
                color: Color::from_rgb(0.26, 0.47, 0.69),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::from_rgb(0.1, 0.1, 0.1),
            shadow: Shadow::default(), // No shadow when pressed
            snap: false,
        },
        button::Status::Disabled => button::Style {
            // Grayed out appearance
            background: Some(Background::Gradient(iced::Gradient::Linear(
                iced::gradient::Linear::new(iced::Radians(1.5708))
                    .add_stop(0.0, Color::from_rgb(0.92, 0.92, 0.92))
                    .add_stop(1.0, Color::from_rgb(0.85, 0.85, 0.85)),
            ))),
            border: Border {
                color: Color::from_rgb(0.65, 0.65, 0.65),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Color::from_rgb(0.6, 0.6, 0.6), // Grayed text
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

/// Apply opacity to button style for fade animations
pub fn apply_opacity_to_button_style(
    mut style: button::Style,
    opacity: f32,
) -> button::Style {
    // Apply opacity to all colors in the button style
    if let Some(Background::Color(color)) = style.background {
        style.background = Some(Background::Color(Color::from_rgba(
            color.r,
            color.g,
            color.b,
            opacity,
        )));
    } else if let Some(Background::Gradient(gradient)) = style.background {
        // For gradients, we need to apply opacity to each stop
        let iced::Gradient::Linear(linear) = gradient;
        let mut new_linear = iced::gradient::Linear::new(linear.angle);
        for stop_option in linear.stops.iter() {
            if let Some(stop) = stop_option {
                new_linear = new_linear.add_stop(
                    stop.offset,
                    Color::from_rgba(stop.color.r, stop.color.g, stop.color.b, opacity),
                );
            }
        }
        style.background = Some(Background::Gradient(iced::Gradient::Linear(new_linear)));
    }

    // Apply opacity to text and border
    style.text_color = Color::from_rgba(
        style.text_color.r,
        style.text_color.g,
        style.text_color.b,
        opacity,
    );
    style.border.color = Color::from_rgba(
        style.border.color.r,
        style.border.color.g,
        style.border.color.b,
        opacity,
    );
    style.shadow.color = Color::from_rgba(
        style.shadow.color.r,
        style.shadow.color.g,
        style.shadow.color.b,
        style.shadow.color.a * opacity,
    );

    style
}
