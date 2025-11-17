use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Egui Demo - Modern Immediate Mode GUI",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    // Calculator
    calc_display: String,
    calc_current: f64,
    calc_operation: Option<char>,
    calc_waiting_for_operand: bool,

    // Text input
    text_input: String,
    multiline_text: String,

    // Sliders and values
    slider_value: f32,
    age: u8,
    
    // Checkboxes and toggles
    checkbox: bool,
    toggle: bool,

    // Radio buttons
    selected_option: Option<usize>,

    // Color picker
    color: egui::Color32,

    // Progress
    progress: f32,

    // Plot data
    plot_data: Vec<f64>,
    plot_index: usize,

    // Unused fields removed
    _image_scale: f32,
    _image_tint: egui::Color32,

    // Combo box
    selected_combo: String,
    combo_options: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            calc_display: String::from("0"),
            calc_current: 0.0,
            calc_operation: None,
            calc_waiting_for_operand: false,
            text_input: String::from("Hello Egui!"),
            multiline_text: String::from("Multi-line\ntext editor\nsupported!"),
            slider_value: 0.5,
            age: 25,
            checkbox: true,
            toggle: false,
            selected_option: Some(0),
            color: egui::Color32::from_rgb(200, 100, 50),
            progress: 0.0,
            plot_data: vec![0.0; 100],
            plot_index: 0,
            _image_scale: 1.0,
            _image_tint: egui::Color32::WHITE,
            selected_combo: String::from("Option 1"),
            combo_options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
                "Option 4".to_string(),
            ],
        }
    }
}

impl MyApp {
    fn calc_input_digit(&mut self, digit: u8) {
        if self.calc_waiting_for_operand {
            self.calc_display = digit.to_string();
            self.calc_waiting_for_operand = false;
        } else {
            if self.calc_display == "0" {
                self.calc_display = digit.to_string();
            } else {
                self.calc_display.push_str(&digit.to_string());
            }
        }
    }

    fn calc_operation(&mut self, op: char) {
        let current_value: f64 = self.calc_display.parse().unwrap_or(0.0);

        if let Some(prev_op) = self.calc_operation {
            let result = match prev_op {
                '+' => self.calc_current + current_value,
                '-' => self.calc_current - current_value,
                '*' => self.calc_current * current_value,
                '/' => {
                    if current_value != 0.0 {
                        self.calc_current / current_value
                    } else {
                        self.calc_current
                    }
                }
                _ => current_value,
            };
            self.calc_current = result;
            self.calc_display = format!("{}", result);
        } else {
            self.calc_current = current_value;
        }

        self.calc_operation = Some(op);
        self.calc_waiting_for_operand = true;
    }

    fn calc_equals(&mut self) {
        if let Some(op) = self.calc_operation {
            self.calc_operation(op);
            self.calc_operation = None;
        }
    }

    fn calc_clear(&mut self) {
        self.calc_display = String::from("0");
        self.calc_current = 0.0;
        self.calc_operation = None;
        self.calc_waiting_for_operand = false;
    }

    fn update_plot(&mut self) {
        self.plot_data[self.plot_index] = self.slider_value as f64;
        self.plot_index = (self.plot_index + 1) % self.plot_data.len();
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-increment progress
        self.progress = (self.progress + 0.001) % 1.0;

        // Top panel with menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.label("Theme options would go here");
                });
                ui.menu_button("Help", |ui| {
                    ui.label("Egui Demo Application");
                    ui.label("Version 0.1.0");
                });
            });
        });

        // Left sidebar
        egui::SidePanel::left("left_panel")
            .default_width(350.0)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Text input
                    ui.group(|ui| {
                        ui.label("Text Input:");
                        ui.text_edit_singleline(&mut self.text_input);
                        
                        ui.add_space(5.0);
                        ui.label("Multi-line:");
                        ui.text_edit_multiline(&mut self.multiline_text);

                        ui.horizontal(|ui| {
                            if ui.button("Uppercase").clicked() {
                                self.text_input = self.text_input.to_uppercase();
                            }
                            if ui.button("Lowercase").clicked() {
                                self.text_input = self.text_input.to_lowercase();
                            }
                            if ui.button("Clear").clicked() {
                                self.text_input.clear();
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Sliders
                    ui.group(|ui| {
                        ui.label("Sliders:");
                        ui.add(egui::Slider::new(&mut self.slider_value, 0.0..=1.0).text("Value"));
                        ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Age"));
                        ui.label(format!("Slider: {:.2}, Age: {}", self.slider_value, self.age));
                    });

                    ui.add_space(10.0);

                    // Checkboxes and toggles
                    ui.group(|ui| {
                        ui.label("Checkboxes & Toggles:");
                        ui.checkbox(&mut self.checkbox, "Checkbox");
                        ui.toggle_value(&mut self.toggle, "Toggle switch");
                        
                        if self.checkbox {
                            ui.colored_label(egui::Color32::GREEN, "✓ Checkbox is checked");
                        }
                    });

                    ui.add_space(10.0);

                    // Radio buttons
                    ui.group(|ui| {
                        ui.label("Radio Buttons:");
                        ui.radio_value(&mut self.selected_option, Some(0), "Option A");
                        ui.radio_value(&mut self.selected_option, Some(1), "Option B");
                        ui.radio_value(&mut self.selected_option, Some(2), "Option C");
                    });

                    ui.add_space(10.0);

                    // Combo box
                    ui.group(|ui| {
                        ui.label("Combo Box:");
                        egui::ComboBox::from_label("Select option")
                            .selected_text(&self.selected_combo)
                            .show_ui(ui, |ui| {
                                for option in &self.combo_options {
                                    ui.selectable_value(&mut self.selected_combo, option.clone(), option);
                                }
                            });
                    });

                    ui.add_space(10.0);

                    // Color picker
                    ui.group(|ui| {
                        ui.label("Color Picker:");
                        ui.color_edit_button_srgba(&mut self.color);
                        let [r, g, b, a] = self.color.to_array();
                        ui.label(format!("RGBA: ({}, {}, {}, {})", r, g, b, a));
                    });

                    ui.add_space(10.0);

                    // Progress bar
                    ui.group(|ui| {
                        ui.label("Progress Bar:");
                        ui.add(egui::ProgressBar::new(self.progress).show_percentage());
                        ui.label("(Auto-incrementing)");
                    });
                });
            });

        // Right sidebar - Calculator
        egui::SidePanel::right("right_panel")
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.heading("Calculator");
                ui.separator();

                // Display
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new(&self.calc_display)
                            .size(32.0)
                            .monospace(),
                    );
                    ui.add_space(10.0);
                });

                ui.separator();

                // Number pad
                egui::Grid::new("calc_grid")
                    .spacing([5.0, 5.0])
                    .show(ui, |ui| {
                        for row in 0..3 {
                            for col in 0..3 {
                                let digit = (2 - row) * 3 + col + 1;
                                if ui.button(egui::RichText::new(digit.to_string()).size(20.0))
                                    .clicked()
                                {
                                    self.calc_input_digit(digit as u8);
                                }
                            }
                            // Operations on the right
                            let op = match row {
                                0 => "+",
                                1 => "-",
                                2 => "*",
                                _ => "",
                            };
                            if ui.button(egui::RichText::new(op).size(20.0)).clicked() {
                                self.calc_operation(op.chars().next().unwrap());
                            }
                            ui.end_row();
                        }

                        // Bottom row
                        if ui.button(egui::RichText::new("C").size(20.0)).clicked() {
                            self.calc_clear();
                        }
                        if ui.button(egui::RichText::new("0").size(20.0)).clicked() {
                            self.calc_input_digit(0);
                        }
                        if ui.button(egui::RichText::new("=").size(20.0)).clicked() {
                            self.calc_equals();
                        }
                        if ui.button(egui::RichText::new("/").size(20.0)).clicked() {
                            self.calc_operation('/');
                        }
                    });
            });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Visualization & Graphics");
            ui.separator();

            // Plot
            ui.group(|ui| {
                ui.label("Slider Value History:");
                
                let points: PlotPoints = self
                    .plot_data
                    .iter()
                    .enumerate()
                    .map(|(i, &y)| [i as f64, y])
                    .collect();

                Plot::new("plot")
                    .height(200.0)
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(Line::new("Slider History", points).color(egui::Color32::from_rgb(100, 200, 100)));
                    });

                if ui.button("Update Plot").clicked() {
                    self.update_plot();
                }
            });

            ui.add_space(20.0);

            // Color preview
            ui.group(|ui| {
                ui.label("Selected Color Preview:");
                let (rect, _response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 150.0),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(rect, 5.0, self.color);
            });

            ui.add_space(20.0);

            // Custom painting
            ui.group(|ui| {
                ui.label("Custom Drawing:");
                let (response, painter) = ui.allocate_painter(
                    egui::vec2(ui.available_width(), 200.0),
                    egui::Sense::hover(),
                );

                let rect = response.rect;
                let center = rect.center();

                // Background
                painter.rect_filled(rect, 5.0, egui::Color32::from_gray(30));

                // Draw circles
                for i in 0..10 {
                    let angle = (i as f32) * std::f32::consts::TAU / 10.0;
                    let radius = 60.0;
                    let pos = center + egui::vec2(angle.cos() * radius, angle.sin() * radius);
                    let color_val = (i as f32 / 10.0 * 255.0) as u8;
                    painter.circle_filled(
                        pos,
                        10.0,
                        egui::Color32::from_rgb(color_val, 100, 255 - color_val),
                    );
                }

                // Draw lines connecting to mouse
                if let Some(mouse_pos) = response.hover_pos() {
                    for i in 0..10 {
                        let angle = (i as f32) * std::f32::consts::TAU / 10.0;
                        let radius = 60.0;
                        let pos = center + egui::vec2(angle.cos() * radius, angle.sin() * radius);
                        painter.line_segment(
                            [pos, mouse_pos],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 255)),
                        );
                    }
                }

                // Center circle
                painter.circle_filled(center, 15.0, egui::Color32::WHITE);
            });
        });

        // Bottom panel with info
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Egui Demo");
                ui.separator();
                ui.label(format!("FPS: {:.1}", ctx.input(|i| i.stable_dt.recip())));
                ui.separator();
                ui.hyperlink_to("Egui Documentation", "https://docs.rs/egui");
            });
        });
    }
}
