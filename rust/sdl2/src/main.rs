use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const BALL_SIZE: u32 = 20;
const PADDLE_WIDTH: u32 = 15;
const PADDLE_HEIGHT: u32 = 100;
const PARTICLE_COUNT: usize = 100;

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
}

impl Ball {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vel_x: 300.0,
            vel_y: 200.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;

        // Bounce off top and bottom
        if self.y <= 0.0 || self.y >= (WINDOW_HEIGHT - BALL_SIZE) as f32 {
            self.vel_y = -self.vel_y;
            self.y = self.y.clamp(0.0, (WINDOW_HEIGHT - BALL_SIZE) as f32);
        }
    }

    fn reset(&mut self) {
        self.x = (WINDOW_WIDTH / 2) as f32;
        self.y = (WINDOW_HEIGHT / 2) as f32;
        self.vel_x = if rand::rng().random_bool(0.5) {
            300.0
        } else {
            -300.0
        };
        self.vel_y = rand::rng().random_range(-200.0..200.0);
    }
}

struct Paddle {
    y: f32,
    vel_y: f32,
}

impl Paddle {
    fn new(y: f32) -> Self {
        Self { y, vel_y: 0.0 }
    }

    fn update(&mut self, dt: f32) {
        self.y += self.vel_y * dt;
        self.y = self
            .y
            .clamp(0.0, (WINDOW_HEIGHT - PADDLE_HEIGHT) as f32);
    }
}

struct Particle {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    lifetime: f32,
    max_lifetime: f32,
    color: Color,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::rng();
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let speed = rng.random_range(50.0..200.0);
        Self {
            x,
            y,
            vel_x: angle.cos() * speed,
            vel_y: angle.sin() * speed,
            lifetime: 0.0,
            max_lifetime: rng.random_range(0.5..2.0),
            color: Color::RGB(
                rng.random_range(100..255),
                rng.random_range(100..255),
                rng.random_range(100..255),
            ),
        }
    }

    fn update(&mut self, dt: f32) -> bool {
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;
        self.lifetime += dt;
        self.lifetime < self.max_lifetime
    }

    fn alpha(&self) -> u8 {
        ((1.0 - self.lifetime / self.max_lifetime) * 255.0) as u8
    }
}

struct GameState {
    ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,
    left_score: u32,
    right_score: u32,
    particles: Vec<Particle>,
    show_trail: bool,
    trail_points: Vec<Point>,
}

impl GameState {
    fn new() -> Self {
        Self {
            ball: Ball::new((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32),
            left_paddle: Paddle::new((WINDOW_HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32),
            right_paddle: Paddle::new((WINDOW_HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32),
            left_score: 0,
            right_score: 0,
            particles: Vec::new(),
            show_trail: true,
            trail_points: Vec::new(),
        }
    }

    fn update(&mut self, dt: f32) {
        self.ball.update(dt);
        self.left_paddle.update(dt);
        self.right_paddle.update(dt);

        // Update particles
        self.particles.retain_mut(|p| p.update(dt));

        // Add trail point
        if self.show_trail {
            self.trail_points.push(Point::new(
                (self.ball.x + BALL_SIZE as f32 / 2.0) as i32,
                (self.ball.y + BALL_SIZE as f32 / 2.0) as i32,
            ));
            if self.trail_points.len() > 50 {
                self.trail_points.remove(0);
            }
        }

        // Check paddle collisions
        let ball_rect = Rect::new(
            self.ball.x as i32,
            self.ball.y as i32,
            BALL_SIZE,
            BALL_SIZE,
        );

        let left_paddle_rect = Rect::new(
            20,
            self.left_paddle.y as i32,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );

        let right_paddle_rect = Rect::new(
            (WINDOW_WIDTH - 20 - PADDLE_WIDTH) as i32,
            self.right_paddle.y as i32,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );

        if ball_rect.has_intersection(left_paddle_rect) && self.ball.vel_x < 0.0 {
            self.ball.vel_x = -self.ball.vel_x * 1.05;
            self.ball.x = (20 + PADDLE_WIDTH) as f32;
            self.spawn_particles(self.ball.x, self.ball.y + BALL_SIZE as f32 / 2.0);
        }

        if ball_rect.has_intersection(right_paddle_rect) && self.ball.vel_x > 0.0 {
            self.ball.vel_x = -self.ball.vel_x * 1.05;
            self.ball.x = (WINDOW_WIDTH - 20 - PADDLE_WIDTH - BALL_SIZE) as f32;
            self.spawn_particles(self.ball.x, self.ball.y + BALL_SIZE as f32 / 2.0);
        }

        // Check scoring
        if self.ball.x <= 0.0 {
            self.right_score += 1;
            self.ball.reset();
            self.trail_points.clear();
        } else if self.ball.x >= (WINDOW_WIDTH - BALL_SIZE) as f32 {
            self.left_score += 1;
            self.ball.reset();
            self.trail_points.clear();
        }
    }

    fn spawn_particles(&mut self, x: f32, y: f32) {
        for _ in 0..20 {
            self.particles.push(Particle::new(x, y));
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2 Demo - Pong", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut game_state = GameState::new();

    let mut last_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    let mut current_fps = 0;

    'running: loop {
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32();
        last_time = now;

        // FPS counter
        fps_counter += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            current_fps = fps_counter;
            fps_counter = 0;
            fps_timer = Instant::now();
        }

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => game_state.left_paddle.vel_y = -400.0,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => game_state.left_paddle.vel_y = 400.0,
                Event::KeyUp {
                    keycode: Some(Keycode::W | Keycode::S),
                    ..
                } => game_state.left_paddle.vel_y = 0.0,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => game_state.right_paddle.vel_y = -400.0,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => game_state.right_paddle.vel_y = 400.0,
                Event::KeyUp {
                    keycode: Some(Keycode::Up | Keycode::Down),
                    ..
                } => game_state.right_paddle.vel_y = 0.0,
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => game_state.show_trail = !game_state.show_trail,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    game_state.spawn_particles(
                        game_state.ball.x + BALL_SIZE as f32 / 2.0,
                        game_state.ball.y + BALL_SIZE as f32 / 2.0,
                    );
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    game_state = GameState::new();
                }
                _ => {}
            }
        }

        // Update game state
        game_state.update(dt);

        // Render
        canvas.set_draw_color(Color::RGB(20, 20, 30));
        canvas.clear();

        // Draw center line
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        for i in (0..WINDOW_HEIGHT).step_by(20) {
            canvas.fill_rect(Rect::new(
                (WINDOW_WIDTH / 2 - 2) as i32,
                i as i32,
                4,
                10,
            ))?;
        }

        // Draw trail
        if game_state.show_trail && game_state.trail_points.len() > 1 {
            for i in 0..game_state.trail_points.len() - 1 {
                let alpha = ((i as f32 / game_state.trail_points.len() as f32) * 255.0) as u8;
                canvas.set_draw_color(Color::RGBA(100, 200, 255, alpha));
                canvas.draw_line(game_state.trail_points[i], game_state.trail_points[i + 1])?;
            }
        }

        // Draw paddles
        canvas.set_draw_color(Color::RGB(100, 200, 100));
        canvas.fill_rect(Rect::new(
            20,
            game_state.left_paddle.y as i32,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ))?;

        canvas.set_draw_color(Color::RGB(200, 100, 100));
        canvas.fill_rect(Rect::new(
            (WINDOW_WIDTH - 20 - PADDLE_WIDTH) as i32,
            game_state.right_paddle.y as i32,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ))?;

        // Draw ball
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(
            game_state.ball.x as i32,
            game_state.ball.y as i32,
            BALL_SIZE,
            BALL_SIZE,
        ))?;

        // Draw particles
        for particle in &game_state.particles {
            let mut color = particle.color;
            color.a = particle.alpha();
            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(particle.x as i32, particle.y as i32, 4, 4))?;
        }

        // Draw scores (simple rectangles as digits)
        draw_score(&mut canvas, game_state.left_score, WINDOW_WIDTH / 4)?;
        draw_score(&mut canvas, game_state.right_score, 3 * WINDOW_WIDTH / 4)?;

        // Draw FPS and instructions
        draw_text_simple(
            &mut canvas,
            &format!("FPS: {}", current_fps),
            10,
            10,
            Color::RGB(200, 200, 200),
        )?;

        draw_text_simple(
            &mut canvas,
            "W/S: Left Paddle | Up/Down: Right Paddle",
            10,
            WINDOW_HEIGHT - 60,
            Color::RGB(150, 150, 150),
        )?;

        draw_text_simple(
            &mut canvas,
            "T: Toggle Trail | Space: Particles | R: Reset | ESC: Quit",
            10,
            WINDOW_HEIGHT - 30,
            Color::RGB(150, 150, 150),
        )?;

        canvas.present();
    }

    Ok(())
}

fn draw_score(canvas: &mut Canvas<Window>, score: u32, center_x: u32) -> Result<(), String> {
    let digit_width = 40;
    let _digit_height = 60;
    let y = 50;

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Simple 7-segment style display
    let segments = get_digit_segments(score % 10);
    let x = (center_x - digit_width / 2) as i32;

    // Draw segments
    if segments[0] {
        canvas.fill_rect(Rect::new(x + 5, y, 30, 5))?;
    } // top
    if segments[1] {
        canvas.fill_rect(Rect::new(x + 35, y + 5, 5, 25))?;
    } // top-right
    if segments[2] {
        canvas.fill_rect(Rect::new(x + 35, y + 30, 5, 25))?;
    } // bottom-right
    if segments[3] {
        canvas.fill_rect(Rect::new(x + 5, y + 55, 30, 5))?;
    } // bottom
    if segments[4] {
        canvas.fill_rect(Rect::new(x, y + 30, 5, 25))?;
    } // bottom-left
    if segments[5] {
        canvas.fill_rect(Rect::new(x, y + 5, 5, 25))?;
    } // top-left
    if segments[6] {
        canvas.fill_rect(Rect::new(x + 5, y + 28, 30, 5))?;
    } // middle

    Ok(())
}

fn get_digit_segments(digit: u32) -> [bool; 7] {
    match digit {
        0 => [true, true, true, true, true, true, false],
        1 => [false, true, true, false, false, false, false],
        2 => [true, true, false, true, true, false, true],
        3 => [true, true, true, true, false, false, true],
        4 => [false, true, true, false, false, true, true],
        5 => [true, false, true, true, false, true, true],
        6 => [true, false, true, true, true, true, true],
        7 => [true, true, true, false, false, false, false],
        8 => [true, true, true, true, true, true, true],
        9 => [true, true, true, true, false, true, true],
        _ => [false; 7],
    }
}

fn draw_text_simple(
    canvas: &mut Canvas<Window>,
    text: &str,
    x: i32,
    y: u32,
    color: Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    // Simple text rendering using rectangles (very basic)
    // In a real app, you'd use SDL2_ttf for proper text rendering
    let char_width = 8;
    let char_height = 12;

    for (i, _c) in text.chars().enumerate() {
        let rect = Rect::new(x + (i as i32 * char_width), y as i32, 6, char_height);
        canvas.draw_rect(rect)?;
    }

    Ok(())
}
