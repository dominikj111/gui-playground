# ImGui Rust Demo

A simple demonstration of ImGui in Rust featuring interactive buttons, text manipulation, and custom drawing.

## Features

- **Counter Controls**: Increment, decrement, and reset buttons
- **Text Manipulation**: Input field with uppercase, lowercase, and clear operations
- **Interactive Controls**: Slider and checkbox
- **Color Picker**: RGB color selection with live preview
- **Custom Drawing**: Canvas with dynamic point plotting and shape rendering
- **Performance Info**: Real-time FPS and frame time display

## Building and Running

```bash
cargo build
cargo run
```

## Requirements

- Rust 1.70 or later
- OpenGL support

## Controls

### Control Panel Window

- Use the counter buttons to modify the counter value
- Type in the text input and use transformation buttons
- Adjust the slider and toggle the checkbox
- Pick a color using the color picker

### Drawing Canvas Window

- Click "Add Random Point" to add points in a spiral pattern
- Points are connected with lines in the selected color
- Click "Clear Points" to reset the canvas

## Dependencies

- `imgui` - Immediate mode GUI library
- `imgui-glium-renderer` - OpenGL renderer for ImGui
- `imgui-winit-support` - Window integration
- `glium` - OpenGL wrapper
