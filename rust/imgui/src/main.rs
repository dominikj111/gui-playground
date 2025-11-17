use glium::glutin::surface::WindowSurface;
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

struct AppState {
    counter: i32,
    text_input: String,
    slider_value: f32,
    checkbox_value: bool,
    color: [f32; 3],
    draw_list_points: Vec<[f32; 2]>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            counter: 0,
            text_input: String::from("Hello, ImGui!"),
            slider_value: 0.5,
            checkbox_value: false,
            color: [1.0, 0.5, 0.0],
            draw_list_points: Vec::new(),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let (window, display) = create_window(&event_loop);

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(FontConfig {
            size_pixels: font_size,
            ..FontConfig::default()
        }),
    }]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut app_state = AppState::default();

    event_loop
        .run(move |event, window_target| {
            match event {
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::AboutToWait => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let ui = imgui.frame();

                    // Build the UI
                    build_ui(&ui, &mut app_state);

                    let mut target = display.draw();
                    target.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
                    platform.prepare_render(&ui, &window);
                    let draw_data = imgui.render();
                    renderer
                        .render(&mut target, draw_data)
                        .expect("Rendering failed");
                    target.finish().expect("Failed to swap buffers");
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    window_target.exit();
                }
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}

fn create_window(event_loop: &EventLoop<()>) -> (winit::window::Window, Display<WindowSurface>) {
    let window_builder = WindowBuilder::new()
        .with_title("ImGui Rust Demo")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));

    let (window, cfg) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(event_loop);

    (window, cfg)
}

fn build_ui(ui: &imgui::Ui, state: &mut AppState) {
    // Main window with buttons and controls
    ui.window("Control Panel")
        .size([400.0, 500.0], imgui::Condition::FirstUseEver)
        .position([20.0, 20.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Welcome to ImGui Rust Demo!");
            ui.separator();

            // Counter section
            ui.text(format!("Counter: {}", state.counter));
            if ui.button("Increment") {
                state.counter += 1;
            }
            ui.same_line();
            if ui.button("Decrement") {
                state.counter -= 1;
            }
            ui.same_line();
            if ui.button("Reset") {
                state.counter = 0;
            }

            ui.separator();

            // Text input section
            ui.text("Text Manipulation:");
            ui.input_text("Input", &mut state.text_input).build();

            if ui.button("Uppercase") {
                state.text_input = state.text_input.to_uppercase();
            }
            ui.same_line();
            if ui.button("Lowercase") {
                state.text_input = state.text_input.to_lowercase();
            }
            ui.same_line();
            if ui.button("Clear") {
                state.text_input.clear();
            }

            ui.text(format!("Length: {} characters", state.text_input.len()));

            ui.separator();

            // Slider and checkbox
            ui.slider("Slider", 0.0, 1.0, &mut state.slider_value);
            ui.checkbox("Checkbox", &mut state.checkbox_value);

            if state.checkbox_value {
                ui.text_colored([0.0, 1.0, 0.0, 1.0], "Checkbox is checked!");
            }

            ui.separator();

            // Color picker
            ui.text("Color Picker:");
            ui.color_edit3("Color", &mut state.color);

            // Display colored text
            ui.text_colored(
                [state.color[0], state.color[1], state.color[2], 1.0],
                "This text uses the selected color!",
            );

            ui.separator();

            // Drawing controls
            ui.text("Drawing:");
            if ui.button("Add Random Point") {
                use std::f32::consts::PI;
                let angle = (state.draw_list_points.len() as f32) * 0.5;
                let radius = 50.0 + (state.draw_list_points.len() as f32) * 2.0;
                state
                    .draw_list_points
                    .push([angle.cos() * radius, angle.sin() * radius]);
            }
            ui.same_line();
            if ui.button("Clear Points") {
                state.draw_list_points.clear();
            }

            ui.text(format!("Points: {}", state.draw_list_points.len()));
        });

    // Drawing window
    ui.window("Drawing Canvas")
        .size([400.0, 500.0], imgui::Condition::FirstUseEver)
        .position([440.0, 20.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Canvas for custom drawing");
            ui.separator();

            let draw_list = ui.get_window_draw_list();
            let canvas_pos = ui.cursor_screen_pos();
            let canvas_size = [350.0, 400.0];

            // Draw background
            draw_list
                .add_rect(
                    canvas_pos,
                    [
                        canvas_pos[0] + canvas_size[0],
                        canvas_pos[1] + canvas_size[1],
                    ],
                    [0.2, 0.2, 0.2, 1.0],
                )
                .filled(true)
                .build();

            // Draw border
            draw_list
                .add_rect(
                    canvas_pos,
                    [
                        canvas_pos[0] + canvas_size[0],
                        canvas_pos[1] + canvas_size[1],
                    ],
                    [1.0, 1.0, 1.0, 1.0],
                )
                .thickness(2.0)
                .build();

            // Draw center point
            let center = [
                canvas_pos[0] + canvas_size[0] / 2.0,
                canvas_pos[1] + canvas_size[1] / 2.0,
            ];

            draw_list
                .add_circle(center, 5.0, [1.0, 1.0, 1.0, 1.0])
                .filled(true)
                .build();

            // Draw points and connect them
            for (i, point) in state.draw_list_points.iter().enumerate() {
                let pos = [center[0] + point[0], center[1] + point[1]];

                // Draw circle at point
                draw_list
                    .add_circle(pos, 4.0, state.color)
                    .filled(true)
                    .build();

                // Connect to previous point
                if i > 0 {
                    let prev_point = &state.draw_list_points[i - 1];
                    let prev_pos = [center[0] + prev_point[0], center[1] + prev_point[1]];
                    draw_list
                        .add_line(prev_pos, pos, state.color)
                        .thickness(2.0)
                        .build();
                }
            }

            // Connect last to first if we have points
            if state.draw_list_points.len() > 2 {
                let first_point = &state.draw_list_points[0];
                let last_point = &state.draw_list_points[state.draw_list_points.len() - 1];
                let first_pos = [center[0] + first_point[0], center[1] + first_point[1]];
                let last_pos = [center[0] + last_point[0], center[1] + last_point[1]];
                draw_list
                    .add_line(last_pos, first_pos, state.color)
                    .thickness(2.0)
                    .build();
            }

            // Draw some decorative elements
            draw_list
                .add_rect(
                    [canvas_pos[0] + 10.0, canvas_pos[1] + 10.0],
                    [canvas_pos[0] + 50.0, canvas_pos[1] + 50.0],
                    [0.0, 1.0, 1.0, 0.5],
                )
                .filled(true)
                .build();

            draw_list
                .add_triangle(
                    [canvas_pos[0] + canvas_size[0] - 50.0, canvas_pos[1] + 10.0],
                    [canvas_pos[0] + canvas_size[0] - 10.0, canvas_pos[1] + 10.0],
                    [canvas_pos[0] + canvas_size[0] - 30.0, canvas_pos[1] + 50.0],
                    [1.0, 1.0, 0.0, 0.5],
                )
                .filled(true)
                .build();
        });

    // Info window
    ui.window("Info")
        .size([200.0, 100.0], imgui::Condition::FirstUseEver)
        .position([860.0, 20.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("FPS: {:.1}", ui.io().framerate));
            ui.text(format!("Frame time: {:.3} ms", 1000.0 / ui.io().framerate));
        });
}
