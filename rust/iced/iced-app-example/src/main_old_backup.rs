mod bar_chart;

use bar_chart::{BarChart, BarData};
use iced::Center;
use iced::time::{self, milliseconds};
use iced::widget::{
    Column, button, checkbox, column, container, pick_list, row, scrollable, slider, text,
    text_input,
};
use iced::{Background, Border, Color, Shadow, Subscription, Theme, Vector};
use std::time::Instant;

pub fn main() -> iced::Result {
    iced::application(Counter::new, Counter::update, Counter::view)
        .subscription(Counter::subscription)
        .theme(Counter::theme)
        .run()
}

struct Counter {
    value: i64,
    name: String,
    email: String,
    age: u8,
    experience: f32,
    selected_role: Option<Role>,
    selected_theme: Theme,
    chart: BarChart,
    current_view: ViewMode,
    previous_view: ViewMode,
    transition_start: Option<Instant>,
    second_button_visible: bool,
    button_animation_start: Option<Instant>,
    button_animation_direction: AnimationDirection,
    // Browser form fields
    browser_text: String,
    browser_password: String,
    browser_slider_value: f32,
    browser_checkbox: bool,
    browser_selected_option: Option<BrowserOption>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnimationDirection {
    FadeIn,
    FadeOut,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Form,
    Chart,
    Summary,
    BrowserForm,
}

impl Counter {
    fn new() -> (Self, iced::Task<Message>) {
        (Self::default(), iced::Task::none())
    }
}

impl Default for Counter {
    fn default() -> Self {
        let chart_data = vec![
            BarData {
                label: "Counter".to_string(),
                value: 0.0,
                color: Color::from_rgb(0.2, 0.6, 0.9),
            },
            BarData {
                label: "Age".to_string(),
                value: 0.0,
                color: Color::from_rgb(0.9, 0.4, 0.2),
            },
            BarData {
                label: "Experience".to_string(),
                value: 0.0,
                color: Color::from_rgb(0.3, 0.8, 0.3),
            },
        ];

        Self {
            value: 0,
            name: String::new(),
            email: String::new(),
            age: 0,
            experience: 0.0,
            selected_role: None,
            selected_theme: Theme::Light,
            chart: BarChart::new(chart_data),
            current_view: ViewMode::Form,
            previous_view: ViewMode::Form,
            transition_start: None,
            second_button_visible: false,
            button_animation_start: None,
            button_animation_direction: AnimationDirection::None,
            browser_text: String::new(),
            browser_password: String::new(),
            browser_slider_value: 50.0,
            browser_checkbox: false,
            browser_selected_option: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Developer,
    Designer,
    Manager,
    Tester,
}

impl Role {
    const ALL: [Role; 4] = [Role::Developer, Role::Designer, Role::Manager, Role::Tester];
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::Developer => "Developer",
                Role::Designer => "Designer",
                Role::Manager => "Manager",
                Role::Tester => "Tester",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrowserOption {
    OptionA,
    OptionB,
    OptionC,
}

impl BrowserOption {
    const ALL: [BrowserOption; 3] = [
        BrowserOption::OptionA,
        BrowserOption::OptionB,
        BrowserOption::OptionC,
    ];
}

impl std::fmt::Display for BrowserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BrowserOption::OptionA => "Option A",
                BrowserOption::OptionB => "Option B",
                BrowserOption::OptionC => "Option C",
            }
        )
    }
}

// Define a custom Windows 7 style button function with animated graying
fn windows_7_button_style_with_gray(
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

// Define a custom Windows 7 style button function
fn windows_7_button_style(theme: &Theme, status: button::Status) -> button::Style {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeType {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl ThemeType {
    const ALL: [ThemeType; 22] = [
        ThemeType::Light,
        ThemeType::Dark,
        ThemeType::Dracula,
        ThemeType::Nord,
        ThemeType::SolarizedLight,
        ThemeType::SolarizedDark,
        ThemeType::GruvboxLight,
        ThemeType::GruvboxDark,
        ThemeType::CatppuccinLatte,
        ThemeType::CatppuccinFrappe,
        ThemeType::CatppuccinMacchiato,
        ThemeType::CatppuccinMocha,
        ThemeType::TokyoNight,
        ThemeType::TokyoNightStorm,
        ThemeType::TokyoNightLight,
        ThemeType::KanagawaWave,
        ThemeType::KanagawaDragon,
        ThemeType::KanagawaLotus,
        ThemeType::Moonfly,
        ThemeType::Nightfly,
        ThemeType::Oxocarbon,
        ThemeType::Ferra,
    ];

    fn to_theme(self) -> Theme {
        match self {
            ThemeType::Light => Theme::Light,
            ThemeType::Dark => Theme::Dark,
            ThemeType::Dracula => Theme::Dracula,
            ThemeType::Nord => Theme::Nord,
            ThemeType::SolarizedLight => Theme::SolarizedLight,
            ThemeType::SolarizedDark => Theme::SolarizedDark,
            ThemeType::GruvboxLight => Theme::GruvboxLight,
            ThemeType::GruvboxDark => Theme::GruvboxDark,
            ThemeType::CatppuccinLatte => Theme::CatppuccinLatte,
            ThemeType::CatppuccinFrappe => Theme::CatppuccinFrappe,
            ThemeType::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            ThemeType::CatppuccinMocha => Theme::CatppuccinMocha,
            ThemeType::TokyoNight => Theme::TokyoNight,
            ThemeType::TokyoNightStorm => Theme::TokyoNightStorm,
            ThemeType::TokyoNightLight => Theme::TokyoNightLight,
            ThemeType::KanagawaWave => Theme::KanagawaWave,
            ThemeType::KanagawaDragon => Theme::KanagawaDragon,
            ThemeType::KanagawaLotus => Theme::KanagawaLotus,
            ThemeType::Moonfly => Theme::Moonfly,
            ThemeType::Nightfly => Theme::Nightfly,
            ThemeType::Oxocarbon => Theme::Oxocarbon,
            ThemeType::Ferra => Theme::Ferra,
        }
    }
}

impl std::fmt::Display for ThemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ThemeType::Light => "Light",
                ThemeType::Dark => "Dark",
                ThemeType::Dracula => "Dracula",
                ThemeType::Nord => "Nord",
                ThemeType::SolarizedLight => "Solarized Light",
                ThemeType::SolarizedDark => "Solarized Dark",
                ThemeType::GruvboxLight => "Gruvbox Light",
                ThemeType::GruvboxDark => "Gruvbox Dark",
                ThemeType::CatppuccinLatte => "Catppuccin Latte",
                ThemeType::CatppuccinFrappe => "Catppuccin Frappé",
                ThemeType::CatppuccinMacchiato => "Catppuccin Macchiato",
                ThemeType::CatppuccinMocha => "Catppuccin Mocha",
                ThemeType::TokyoNight => "Tokyo Night",
                ThemeType::TokyoNightStorm => "Tokyo Night Storm",
                ThemeType::TokyoNightLight => "Tokyo Night Light",
                ThemeType::KanagawaWave => "Kanagawa Wave",
                ThemeType::KanagawaDragon => "Kanagawa Dragon",
                ThemeType::KanagawaLotus => "Kanagawa Lotus",
                ThemeType::Moonfly => "Moonfly",
                ThemeType::Nightfly => "Nightfly",
                ThemeType::Oxocarbon => "Oxocarbon",
                ThemeType::Ferra => "Ferra",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    NameChanged(String),
    EmailChanged(String),
    AgeChanged(u8),
    ExperienceChanged(f32),
    RoleSelected(Role),
    ThemeSelected(ThemeType),
    ViewChanged(ViewMode),
    Tick(Instant),
    ShowSecondButton,
    HideSecondButton,
    // Browser form messages
    BrowserTextChanged(String),
    BrowserPasswordChanged(String),
    BrowserSliderChanged(f32),
    BrowserCheckboxToggled(bool),
    BrowserOptionSelected(BrowserOption),
    BrowserFormSubmit,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::NameChanged(name) => {
                self.name = name;
            }
            Message::EmailChanged(email) => {
                self.email = email;
            }
            Message::AgeChanged(age) => {
                self.age = age;
            }
            Message::ExperienceChanged(experience) => {
                self.experience = experience;
            }
            Message::RoleSelected(role) => {
                self.selected_role = Some(role);
            }
            Message::ThemeSelected(theme) => {
                self.selected_theme = theme.to_theme();
            }
            Message::ViewChanged(view) => {
                if self.current_view != view {
                    self.previous_view = self.current_view;
                    self.current_view = view;
                    self.transition_start = Some(Instant::now());
                }
            }
            Message::Tick(_now) => {
                // Animation tick - will trigger redraws during transition
                // Also check if button animation is complete
                if let Some(start) = self.button_animation_start {
                    let elapsed = start.elapsed().as_millis() as f32;
                    let duration = 300.0; // 300ms animation
                    if elapsed >= duration {
                        // Animation complete
                        match self.button_animation_direction {
                            AnimationDirection::FadeIn => {
                                self.button_animation_start = None;
                                self.button_animation_direction = AnimationDirection::None;
                            }
                            AnimationDirection::FadeOut => {
                                self.second_button_visible = false;
                                self.button_animation_start = None;
                                self.button_animation_direction = AnimationDirection::None;
                            }
                            AnimationDirection::None => {}
                        }
                    }
                }
            }
            Message::ShowSecondButton => {
                self.second_button_visible = true;
                self.button_animation_start = Some(Instant::now());
                self.button_animation_direction = AnimationDirection::FadeIn;
            }
            Message::HideSecondButton => {
                self.button_animation_start = Some(Instant::now());
                self.button_animation_direction = AnimationDirection::FadeOut;
            }
            Message::BrowserTextChanged(text) => {
                self.browser_text = text;
            }
            Message::BrowserPasswordChanged(password) => {
                self.browser_password = password;
            }
            Message::BrowserSliderChanged(value) => {
                self.browser_slider_value = value;
            }
            Message::BrowserCheckboxToggled(checked) => {
                self.browser_checkbox = checked;
            }
            Message::BrowserOptionSelected(option) => {
                self.browser_selected_option = Some(option);
            }
            Message::BrowserFormSubmit => {
                // Form submitted - you can add custom logic here
                // For now, it's just a demonstration that the button works
            }
        }

        // Update chart data whenever values change
        self.update_chart();
    }

    fn update_chart(&mut self) {
        let chart_data = vec![
            BarData {
                label: "Counter".to_string(),
                value: self.value.abs() as f32,
                color: Color::from_rgb(0.2, 0.6, 0.9),
            },
            BarData {
                label: "Age".to_string(),
                value: self.age as f32,
                color: Color::from_rgb(0.9, 0.4, 0.2),
            },
            BarData {
                label: "Experience".to_string(),
                value: self.experience,
                color: Color::from_rgb(0.3, 0.8, 0.3),
            },
        ];
        self.chart.update_data(chart_data);
    }

    fn view(&self) -> Column<'_, Message> {
        let menu = self.create_menu();
        let content = self.create_content_with_transition();

        column![
            row![
                menu,
                container(scrollable(content))
                    .width(iced::Fill)
                    .height(iced::Fill)
            ]
            .height(iced::Fill)
        ]
    }

    fn get_transition_progress(&self) -> f32 {
        if let Some(start) = self.transition_start {
            let elapsed = start.elapsed().as_millis() as f32;
            let duration = 300.0; // 300ms transition
            let progress = (elapsed / duration).min(1.0);

            // Ease-out cubic for smooth deceleration
            1.0 - (1.0 - progress).powi(3)
        } else {
            1.0
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // Subscribe to time updates during transitions or button animations
        let needs_animation = (self.transition_start.is_some()
            && self.get_transition_progress() < 1.0)
            || self.button_animation_start.is_some();

        if needs_animation {
            time::every(milliseconds(16)).map(|_| Message::Tick(Instant::now()))
        } else {
            Subscription::none()
        }
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }

    fn get_button_animation_progress(&self) -> f32 {
        if let Some(start) = self.button_animation_start {
            let elapsed = start.elapsed().as_millis() as f32;
            let duration = 300.0; // 300ms animation
            let progress = (elapsed / duration).min(1.0);

            // Ease-out cubic for smooth animation
            1.0 - (1.0 - progress).powi(3)
        } else {
            1.0
        }
    }

    fn get_button_opacity(&self) -> f32 {
        let progress = self.get_button_animation_progress();
        match self.button_animation_direction {
            AnimationDirection::FadeIn => progress,
            AnimationDirection::FadeOut => 1.0 - progress,
            AnimationDirection::None => 1.0,
        }
    }

    fn create_content_with_transition(&self) -> Column<'_, Message> {
        let progress = self.get_transition_progress();
        let content = self.create_content();

        // Apply opacity fade during transition
        if progress < 1.0 {
            // Wrap content in a container with opacity styling
            let alpha = progress;
            column![container(content).style(move |theme: &iced::Theme| {
                let palette = theme.palette();
                container::Style {
                    text_color: Some(Color::from_rgba(
                        palette.text.r,
                        palette.text.g,
                        palette.text.b,
                        alpha,
                    )),
                    background: None,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            })]
        } else {
            column![content]
        }
    }

    fn create_menu(&self) -> Column<'_, Message> {
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

        let current_view_text = match self.current_view {
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

    fn create_content(&self) -> Column<'_, Message> {
        match self.current_view {
            ViewMode::Form => self.create_form_view(),
            ViewMode::Chart => self.create_chart_view(),
            ViewMode::Summary => self.create_summary_view(),
            ViewMode::BrowserForm => self.create_browser_form_view(),
        }
    }

    fn create_form_view(&self) -> Column<'_, Message> {
        column![
            text("Rich Form Application").size(32),
            // Counter section
            container(
                column![
                    text("Counter").size(20),
                    row![
                        button("Decrement").on_press(Message::Decrement),
                        text(self.value).size(30),
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
                    text_input("Enter your name...", &self.name)
                        .on_input(Message::NameChanged)
                        .padding(10),
                    text("Email:").size(14),
                    text_input("Enter your email...", &self.email)
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
                    text(format!("Age: {}", self.age)).size(14),
                    slider(18..=100, self.age, Message::AgeChanged),
                    text(format!("Experience: {:.1} years", self.experience)).size(14),
                    slider(0.0..=20.0, self.experience, Message::ExperienceChanged).step(0.5),
                ]
                .spacing(10)
            )
            .padding(10),
            // Dropdowns section
            container(
                column![
                    text("Preferences").size(20),
                    text("Role:").size(14),
                    pick_list(&Role::ALL[..], self.selected_role, Message::RoleSelected)
                        .placeholder("Choose a role...")
                        .width(250),
                    text("Theme:").size(14),
                    pick_list(
                        &ThemeType::ALL[..],
                        ThemeType::ALL
                            .iter()
                            .find(|t| t.to_theme() == self.selected_theme)
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
            container({
                let is_animating = self.button_animation_start.is_some();
                let opacity = self.get_button_opacity();
                let first_button_enabled = !self.second_button_visible && !is_animating;

                let mut items = vec![];

                // Title
                items.push(text("Animated Buttons Demo").size(20).into());

                // Create button row
                let mut button_row_items = vec![];

                // First button - shows second button when clicked
                // Calculate gray amount for smooth disable animation
                let gray_amount = if first_button_enabled {
                    0.0
                } else if is_animating
                    && self.button_animation_direction == AnimationDirection::FadeIn
                {
                    self.get_button_animation_progress()
                } else if is_animating
                    && self.button_animation_direction == AnimationDirection::FadeOut
                {
                    1.0 - self.get_button_animation_progress()
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
                if self.second_button_visible || is_animating {
                    let second_btn = button("Hide Me").on_press(Message::HideSecondButton).style(
                        move |theme, status| {
                            let mut style = windows_7_button_style(theme, status);

                            // Apply opacity to all colors in the button style
                            if let Some(Background::Color(color)) = style.background {
                                style.background = Some(Background::Color(Color::from_rgba(
                                    color.r, color.g, color.b, opacity,
                                )));
                            } else if let Some(Background::Gradient(gradient)) = style.background {
                                // For gradients, we need to apply opacity to each stop
                                let iced::Gradient::Linear(linear) = gradient;
                                let mut new_linear = iced::gradient::Linear::new(linear.angle);
                                for stop_option in linear.stops.iter() {
                                    if let Some(stop) = stop_option {
                                        new_linear = new_linear.add_stop(
                                            stop.offset,
                                            Color::from_rgba(
                                                stop.color.r,
                                                stop.color.g,
                                                stop.color.b,
                                                opacity,
                                            ),
                                        );
                                    }
                                }
                                style.background =
                                    Some(Background::Gradient(iced::Gradient::Linear(new_linear)));
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
                        },
                    );
                    button_row_items.push(second_btn.into());
                }

                items.push(row(button_row_items).spacing(10).align_y(Center).into());

                column(items).spacing(10)
            })
            .padding(10),
            // Summary section
            container(
                column![
                    text("Summary").size(20),
                    text(format!(
                        "Name: {}",
                        if self.name.is_empty() {
                            "Not set"
                        } else {
                            &self.name
                        }
                    )),
                    text(format!(
                        "Email: {}",
                        if self.email.is_empty() {
                            "Not set"
                        } else {
                            &self.email
                        }
                    )),
                    text(format!("Age: {}", self.age)),
                    text(format!("Experience: {:.1} years", self.experience)),
                    text(format!(
                        "Role: {}",
                        self.selected_role
                            .map(|r| r.to_string())
                            .unwrap_or_else(|| "Not selected".to_string())
                    )),
                    text(format!(
                        "Theme: {}",
                        ThemeType::ALL
                            .iter()
                            .find(|t| t.to_theme() == self.selected_theme)
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "Unknown".to_string())
                    )),
                    text(format!("Counter: {}", self.value)),
                ]
                .spacing(5)
            )
            .padding(10),
            // Bar Chart section
            container(column![text("Data Visualization").size(20), self.chart.view(),].spacing(10))
                .padding(10),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn create_chart_view(&self) -> Column<'_, Message> {
        column![
            text("Data Visualization").size(32),
            container(
                column![
                    text("Interactive Bar Chart").size(20),
                    text("This chart displays your form data in real-time").size(14),
                    self.chart.view(),
                    text(format!("Counter: {}", self.value)).size(16),
                    text(format!("Age: {}", self.age)).size(16),
                    text(format!("Experience: {:.1} years", self.experience)).size(16),
                ]
                .spacing(15)
            )
            .padding(20),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn create_summary_view(&self) -> Column<'_, Message> {
        column![
            text("Summary").size(32),
            container(
                column![
                    text("Personal Information").size(24),
                    text(format!(
                        "Name: {}",
                        if self.name.is_empty() {
                            "Not set"
                        } else {
                            &self.name
                        }
                    ))
                    .size(16),
                    text(format!(
                        "Email: {}",
                        if self.email.is_empty() {
                            "Not set"
                        } else {
                            &self.email
                        }
                    ))
                    .size(16),
                ]
                .spacing(10)
            )
            .padding(20),
            container(
                column![
                    text("Settings").size(24),
                    text(format!("Age: {}", self.age)).size(16),
                    text(format!("Experience: {:.1} years", self.experience)).size(16),
                ]
                .spacing(10)
            )
            .padding(20),
            container(
                column![
                    text("Preferences").size(24),
                    text(format!(
                        "Role: {}",
                        self.selected_role
                            .map(|r| r.to_string())
                            .unwrap_or_else(|| "Not selected".to_string())
                    ))
                    .size(16),
                    text(format!(
                        "Theme: {}",
                        ThemeType::ALL
                            .iter()
                            .find(|t| t.to_theme() == self.selected_theme)
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "Unknown".to_string())
                    ))
                    .size(16),
                ]
                .spacing(10)
            )
            .padding(20),
            container(
                column![
                    text("Counter").size(24),
                    text(format!("Current value: {}", self.value)).size(16),
                ]
                .spacing(10)
            )
            .padding(20),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn create_browser_form_view(&self) -> Column<'_, Message> {
        column![
            text("Browser-Style Form Controls").size(32),
            text("Standard HTML-like form elements with native browser styling")
                .size(14)
                .style(|theme: &Theme| {
                    let palette = theme.palette();
                    text::Style {
                        color: Some(Color::from_rgb(
                            palette.text.r * 0.7,
                            palette.text.g * 0.7,
                            palette.text.b * 0.7,
                        )),
                    }
                }),
            // Text Input Section
            container(
                column![
                    text("Text Input").size(20),
                    text("Username:").size(14),
                    text_input("Enter username...", &self.browser_text)
                        .on_input(Message::BrowserTextChanged)
                        .padding(8)
                        .style(|theme: &Theme, status| {
                            let palette = theme.palette();
                            text_input::Style {
                                background: Background::Color(Color::WHITE),
                                border: Border {
                                    color: match status {
                                        text_input::Status::Active => {
                                            Color::from_rgb(0.7, 0.7, 0.7)
                                        }
                                        text_input::Status::Hovered => {
                                            Color::from_rgb(0.5, 0.5, 0.5)
                                        }
                                        text_input::Status::Focused { .. } => {
                                            Color::from_rgb(0.0, 0.5, 1.0)
                                        }
                                        text_input::Status::Disabled => {
                                            Color::from_rgb(0.8, 0.8, 0.8)
                                        }
                                    },
                                    width: match status {
                                        text_input::Status::Focused { .. } => 2.0,
                                        _ => 1.0,
                                    },
                                    radius: 3.0.into(),
                                },
                                icon: palette.text,
                                placeholder: Color::from_rgb(0.6, 0.6, 0.6),
                                value: Color::BLACK,
                                selection: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
                            }
                        }),
                ]
                .spacing(8)
            )
            .padding(15),
            // Password Input Section
            container(
                column![
                    text("Password Input").size(20),
                    text("Password:").size(14),
                    text_input("Enter password...", &self.browser_password)
                        .on_input(Message::BrowserPasswordChanged)
                        .padding(8)
                        .secure(true)
                        .style(|theme: &Theme, status| {
                            let palette = theme.palette();
                            text_input::Style {
                                background: Background::Color(Color::WHITE),
                                border: Border {
                                    color: match status {
                                        text_input::Status::Active => {
                                            Color::from_rgb(0.7, 0.7, 0.7)
                                        }
                                        text_input::Status::Hovered => {
                                            Color::from_rgb(0.5, 0.5, 0.5)
                                        }
                                        text_input::Status::Focused { .. } => {
                                            Color::from_rgb(0.0, 0.5, 1.0)
                                        }
                                        text_input::Status::Disabled => {
                                            Color::from_rgb(0.8, 0.8, 0.8)
                                        }
                                    },
                                    width: match status {
                                        text_input::Status::Focused { .. } => 2.0,
                                        _ => 1.0,
                                    },
                                    radius: 3.0.into(),
                                },
                                icon: palette.text,
                                placeholder: Color::from_rgb(0.6, 0.6, 0.6),
                                value: Color::BLACK,
                                selection: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
                            }
                        }),
                ]
                .spacing(8)
            )
            .padding(15),
            // Slider Section
            container(
                column![
                    text("Range Slider").size(20),
                    text(format!("Value: {:.0}", self.browser_slider_value)).size(14),
                    slider(
                        0.0..=100.0,
                        self.browser_slider_value,
                        Message::BrowserSliderChanged
                    )
                    .step(1.0)
                    .style(|theme: &Theme, status| {
                        let palette = theme.palette();
                        slider::Style {
                            rail: slider::Rail {
                                backgrounds: (
                                    Background::Color(Color::from_rgb(0.0, 0.5, 1.0)),
                                    Background::Color(Color::from_rgb(0.85, 0.85, 0.85)),
                                ),
                                width: 6.0,
                                border: Border {
                                    color: Color::from_rgb(0.7, 0.7, 0.7),
                                    width: 1.0,
                                    radius: 3.0.into(),
                                },
                            },
                            handle: slider::Handle {
                                shape: slider::HandleShape::Circle { radius: 10.0 },
                                background: Background::Color(match status {
                                    slider::Status::Active => Color::WHITE,
                                    slider::Status::Hovered => Color::from_rgb(0.95, 0.95, 0.95),
                                    slider::Status::Dragged => Color::from_rgb(0.9, 0.9, 0.9),
                                }),
                                border_width: 2.0,
                                border_color: Color::from_rgb(0.6, 0.6, 0.6),
                            },
                        }
                    }),
                ]
                .spacing(8)
            )
            .padding(15),
            // Checkbox Section
            container(
                column![
                    text("Checkbox").size(20),
                    checkbox("I agree to the terms and conditions", self.browser_checkbox)
                        .on_toggle(Message::BrowserCheckboxToggled)
                        .size(20)
                        .spacing(10)
                        .text_size(14)
                        .style(|theme: &Theme, status| {
                            let palette = theme.palette();
                            checkbox::Style {
                                background: Background::Color(if self.browser_checkbox {
                                    Color::from_rgb(0.0, 0.5, 1.0)
                                } else {
                                    Color::WHITE
                                }),
                                icon_color: Color::WHITE,
                                border: Border {
                                    color: match status {
                                        checkbox::Status::Active { .. } => {
                                            Color::from_rgb(0.7, 0.7, 0.7)
                                        }
                                        checkbox::Status::Hovered { .. } => {
                                            Color::from_rgb(0.5, 0.5, 0.5)
                                        }
                                        checkbox::Status::Disabled { .. } => {
                                            Color::from_rgb(0.8, 0.8, 0.8)
                                        }
                                    },
                                    width: 2.0,
                                    radius: 3.0.into(),
                                },
                                text_color: Some(palette.text),
                            }
                        }),
                ]
                .spacing(8)
            )
            .padding(15),
            // Dropdown Section
            container(
                column![
                    text("Select Dropdown").size(20),
                    text("Choose an option:").size(14),
                    pick_list(
                        &BrowserOption::ALL[..],
                        self.browser_selected_option,
                        Message::BrowserOptionSelected
                    )
                    .placeholder("Select an option...")
                    .padding(8)
                    .text_size(14)
                    .style(|theme: &Theme, status| {
                        let palette = theme.palette();
                        pick_list::Style {
                            text_color: Color::BLACK,
                            placeholder_color: Color::from_rgb(0.6, 0.6, 0.6),
                            handle_color: Color::from_rgb(0.4, 0.4, 0.4),
                            background: Background::Color(Color::WHITE),
                            border: Border {
                                color: match status {
                                    pick_list::Status::Active => Color::from_rgb(0.7, 0.7, 0.7),
                                    pick_list::Status::Hovered => Color::from_rgb(0.5, 0.5, 0.5),
                                    pick_list::Status::Opened { .. } => {
                                        Color::from_rgb(0.0, 0.5, 1.0)
                                    }
                                },
                                width: 1.0,
                                radius: 3.0.into(),
                            },
                        }
                    }),
                ]
                .spacing(8)
            )
            .padding(15),
            // Submit Button Section
            container(
                column![
                    text("Submit Button").size(20),
                    button(
                        container(text("Submit Form").size(14))
                            .padding(10)
                            .center_x(iced::Fill)
                    )
                    .width(200)
                    .on_press(Message::BrowserFormSubmit)
                    .style(|_theme: &Theme, status| {
                        button::Style {
                            background: Some(Background::Color(match status {
                                button::Status::Active => Color::from_rgb(0.0, 0.5, 1.0),
                                button::Status::Hovered => Color::from_rgb(0.0, 0.45, 0.9),
                                button::Status::Pressed => Color::from_rgb(0.0, 0.4, 0.8),
                                button::Status::Disabled => Color::from_rgb(0.7, 0.7, 0.7),
                            })),
                            text_color: Color::WHITE,
                            border: Border {
                                color: Color::from_rgb(0.0, 0.4, 0.8),
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            shadow: Shadow {
                                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                                offset: Vector::new(0.0, 2.0),
                                blur_radius: 4.0,
                            },
                            snap: false,
                        }
                    }),
                ]
                .spacing(8)
            )
            .padding(15),
            // Form Summary
            container(
                column![
                    text("Form Data Summary").size(20),
                    text(format!(
                        "Username: {}",
                        if self.browser_text.is_empty() {
                            "Not entered"
                        } else {
                            &self.browser_text
                        }
                    ))
                    .size(14),
                    text(format!(
                        "Password: {}",
                        if self.browser_password.is_empty() {
                            "Not entered"
                        } else {
                            "••••••••"
                        }
                    ))
                    .size(14),
                    text(format!("Slider Value: {:.0}", self.browser_slider_value)).size(14),
                    text(format!(
                        "Checkbox: {}",
                        if self.browser_checkbox {
                            "Checked"
                        } else {
                            "Unchecked"
                        }
                    ))
                    .size(14),
                    text(format!(
                        "Selected Option: {}",
                        self.browser_selected_option
                            .map(|o| o.to_string())
                            .unwrap_or_else(|| "None".to_string())
                    ))
                    .size(14),
                ]
                .spacing(8)
            )
            .padding(15)
            .style(|theme: &Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                    border: Border {
                        color: Color::from_rgb(0.8, 0.8, 0.8),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    text_color: Some(palette.text),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }),
        ]
        .spacing(10)
        .padding(20)
        .align_x(Center)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::{Error, simulator};

    #[test]
    fn it_counts() -> Result<(), Error> {
        let mut counter = Counter::default();
        let mut ui = simulator(counter.view());

        let _ = ui.click("Increment")?;
        let _ = ui.click("Increment")?;
        let _ = ui.click("Decrement")?;

        for message in ui.into_messages() {
            counter.update(message);
        }

        assert_eq!(counter.value, 1);

        let mut ui = simulator(counter.view());
        assert!(ui.find("1").is_ok(), "Counter should display 1!");

        Ok(())
    }
}
