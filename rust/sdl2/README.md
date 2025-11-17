# SDL2 Rust Demo - Pong Game

A classic Pong game implementation showcasing SDL2's capabilities for 2D graphics, animation, and input handling.

Works on:
Raspberry Pi 4 [Debian GNU/Linux 12 (bookworm)]
Linux MX 23 [Debian GNU/Linux 12 (bookworm)] - Radeon HD 6290

## Features

- **Classic Pong Gameplay**: Two-player paddle game with ball physics
- **Particle Effects**: Collision particles with alpha blending
- **Ball Trail**: Visual trail effect showing ball movement
- **Score Display**: 7-segment style score rendering
- **Smooth Animation**: 60 FPS with delta-time based movement
- **Keyboard Controls**: Responsive input handling
- **FPS Counter**: Real-time performance monitoring

## Building and Running

```bash
cargo build
cargo run
```

## System Requirements

SDL2 library must be installed on your system:

### macOS
```bash
brew install sdl2
```

### Linux (Ubuntu/Debian)
```bash
sudo apt-get install libsdl2-dev
```

### Windows
Download SDL2 development libraries from [libsdl.org](https://www.libsdl.org/download-2.0.php)

## Controls

- **W/S**: Move left paddle (green) up/down
- **Up/Down Arrow**: Move right paddle (red) up/down
- **T**: Toggle ball trail effect
- **Space**: Spawn particle burst at ball position
- **R**: Reset game (scores and ball position)
- **ESC**: Quit game

## Game Rules

- Ball bounces off paddles and top/bottom walls
- Ball speeds up slightly with each paddle hit
- Score increases when opponent misses the ball
- First to... well, there's no win condition, just keep playing!

## SDL2 Features Demonstrated

- **Window Management**: Creating and managing game window
- **2D Rendering**: Drawing rectangles, lines, and shapes
- **Event Handling**: Keyboard input processing
- **Game Loop**: Frame-rate independent game logic
- **Color & Alpha**: RGBA color manipulation
- **Collision Detection**: Rectangle intersection testing
- **Particle Systems**: Dynamic particle generation and lifecycle

## Code Structure

- `Ball`: Ball physics and movement
- `Paddle`: Paddle state and controls
- `Particle`: Particle effect system with lifetime
- `GameState`: Overall game state management
- Main loop: Event handling, update, and render cycle

## Extending the Example

Try adding:
- Sound effects using SDL2 mixer
- Menu system
- AI opponent
- Power-ups
- Better text rendering with SDL2_ttf
- Texture-based graphics with SDL2_image
