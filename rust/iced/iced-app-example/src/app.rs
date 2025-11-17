use iced::time::{self, milliseconds};
use iced::widget::{button, column, container, row, scrollable, stack, text, Column};
use iced::{Color, Element, Point, Subscription, Theme};
use std::time::Instant;

use crate::components::{config_menu, menu};
use crate::message::{BrowserOption, Message, Role, ViewMode};
use crate::utils::animation::{AnimationDirection, get_animation_progress};
use crate::widgets::{BarChart, BarData};
use crate::views::{browser_form, chart, form, summary};

pub struct App {
    pub value: i64,
    pub name: String,
    pub email: String,
    pub age: u8,
    pub experience: f32,
    pub selected_role: Option<Role>,
    pub selected_theme: Theme,
    pub chart: BarChart,
    pub current_view: ViewMode,
    pub previous_view: ViewMode,
    pub transition_start: Option<Instant>,
    pub second_button_visible: bool,
    pub button_animation_start: Option<Instant>,
    pub button_animation_direction: AnimationDirection,
    // Browser form fields
    pub browser_text: String,
    pub browser_password: String,
    pub browser_slider_value: f32,
    pub browser_checkbox: bool,
    pub browser_selected_option: Option<BrowserOption>,
    // Config menu state
    pub config_menu_open: bool,
    pub config_menu_animation_start: Option<Instant>,
}

impl App {
    pub fn new() -> (Self, iced::Task<Message>) {
        (Self::default(), iced::Task::none())
    }

    pub fn update(&mut self, message: Message) {
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
                
                // Check if config menu animation is complete
                if let Some(start) = self.config_menu_animation_start {
                    let elapsed = start.elapsed().as_millis() as f32;
                    let duration = 300.0; // 300ms animation
                    if elapsed >= duration {
                        self.config_menu_animation_start = None;
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
                // Handle form submission (e.g., log values, validate, etc.)
                println!("Form submitted!");
                println!("Text: {}", self.browser_text);
                println!("Slider: {}", self.browser_slider_value);
                println!("Checkbox: {}", self.browser_checkbox);
            }
            Message::ToggleConfigMenu => {
                self.config_menu_open = !self.config_menu_open;
                self.config_menu_animation_start = Some(Instant::now());
            }
            Message::ConfigResetData => {
                // Handle reset data action
                println!("Reset All Data clicked");
            }
            Message::ConfigExportSettings => {
                // Handle export settings action
                println!("Export Settings clicked");
            }
            Message::ConfigImportSettings => {
                // Handle import settings action
                println!("Import Settings clicked");
            }
            Message::ConfigMenuInteraction => {
                // Dummy handler to block click-through - does nothing
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

    pub fn view(&self) -> iced::Element<'_, Message> {
        let menu = menu::create_menu(self.current_view);
        let content = self.create_content_with_transition();
        
        // Config menu toggle button (hamburger icon)
        let toggle_button = button(text("☰").size(24))
            .on_press(Message::ToggleConfigMenu)
            .padding(10)
            .style(|_theme: &Theme, _status| {
                button::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 0.8))),
                    border: iced::Border {
                        color: Color::from_rgb(0.4, 0.4, 0.4),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    text_color: Color::WHITE,
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 4.0,
                    },
                    snap: false,
                }
            });

        // Main content
        let main_view = row![
            container(menu).padding(10),
            container(scrollable(content)).padding(20).width(iced::Fill),
        ];
        
        // Config menu (sliding from left)
        let config_menu_widget = config_menu::create_config_menu(
            self.config_menu_open,
            self.config_menu_animation_start,
        );
        
        let offset = config_menu::get_menu_offset(
            self.config_menu_open,
            self.config_menu_animation_start,
        );
        
        // Calculate if menu should be visible
        let menu_visible = self.config_menu_open || self.config_menu_animation_start.is_some();
        
        // Always show menu in stack, but position it off-screen when closed
        let progress = get_animation_progress(self.config_menu_animation_start, 300.0);
        
        // Calculate visible width for sliding effect (0 to 250)
        // Menu stays at 250px but we clip it
        let visible_width = if self.config_menu_open {
            250.0 * progress // Show from 0 to 250
        } else {
            if self.config_menu_animation_start.is_some() {
                250.0 * (1.0 - progress) // Hide from 250 to 0
            } else {
                0.0 // Start hidden
            }
        };
        
        // Create layout with config menu overlay
        {
            // Use stack to overlay config menu
            // Stack layers elements on top of each other without affecting layout
            stack![
                main_view,
                // Full-screen backdrop overlay when menu is visible
                // Use mouse_area with opaque container for better event blocking
                if visible_width > 0.0 {
                    iced::widget::mouse_area(
                        container(
                            container(text(""))
                                .width(iced::Length::Fill)
                                .height(iced::Length::Fill)
                                .style(|_theme| {
                                    container::Style {
                                        background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.01))),
                                        border: iced::Border::default(),
                                        text_color: None,
                                        shadow: iced::Shadow::default(),
                                        snap: false,
                                    }
                                })
                        )
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                    )
                    .on_press(Message::ToggleConfigMenu)
                    .on_enter(Message::ConfigMenuInteraction)
                    .on_move(|_| Message::ConfigMenuInteraction)
                    .interaction(iced::mouse::Interaction::Idle)
                } else {
                    iced::widget::mouse_area(
                        container(text(""))
                            .width(0)
                            .height(0)
                    )
                },
                // Overlay layer for menu - uses row to position from left
                if visible_width > 0.0 {
                    row![
                        // Clipping container for sliding effect
                        container(
                            container(config_menu_widget)
                                .width(250) // Fixed width - menu never changes size
                        )
                        .width(visible_width) // Clip container width animates
                        .height(iced::Length::Fill)
                        .clip(true) // Clip overflow - creates sliding effect
                        .style(move |_theme| {
                            container::Style {
                                background: None,
                                border: iced::Border::default(),
                                text_color: None,
                                shadow: iced::Shadow::default(),
                                snap: false,
                            }
                        }),
                    ]
                    .width(iced::Length::Shrink) // Shrink to menu width
                } else {
                    row![].width(0)
                },
                // Toggle button in top-left corner
                container(toggle_button)
                    .padding(10)
                    .style(|_theme| {
                        container::Style {
                            background: None,
                            border: iced::Border::default(),
                            text_color: None,
                            shadow: iced::Shadow::default(),
                            snap: false,
                        }
                    }),
            ]
            .into()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Subscribe to time updates during transitions, button animations, or menu animation
        let needs_animation = (self.transition_start.is_some()
            && self.get_transition_progress() < 1.0)
            || self.button_animation_start.is_some()
            || self.config_menu_animation_start.is_some();

        if needs_animation {
            time::every(milliseconds(16)).map(|_| Message::Tick(Instant::now()))
        } else {
            Subscription::none()
        }
    }

    pub fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }

    fn get_transition_progress(&self) -> f32 {
        get_animation_progress(self.transition_start, 300.0)
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
                    background: Some(iced::Background::Color(Color::from_rgba(
                        palette.background.r,
                        palette.background.g,
                        palette.background.b,
                        alpha,
                    ))),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            })]
        } else {
            content
        }
    }

    fn create_content(&self) -> Column<'_, Message> {
        match self.current_view {
            ViewMode::Form => form::create_form_view(self),
            ViewMode::Chart => chart::create_chart_view(self),
            ViewMode::Summary => summary::create_summary_view(self),
            ViewMode::BrowserForm => browser_form::create_browser_form_view(self),
        }
    }
}

impl Default for App {
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
            config_menu_open: false,
            config_menu_animation_start: None,
        }
    }
}
