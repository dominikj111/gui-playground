# Egui Demo - Modern Immediate Mode GUI

A comprehensive demonstration of Egui, a modern, pure-Rust immediate mode GUI library that's actively maintained and production-ready.

Works on:
Raspberry Pi 4 [Debian GNU/Linux 12 (bookworm)]
Linux MX 23 [Debian GNU/Linux 12 (bookworm)] - Radeon HD 6290

## Features

### Interactive Widgets

- **Text Input**: Single-line and multi-line text editing
- **Sliders**: Float and integer value controls
- **Checkboxes & Toggles**: Boolean state controls
- **Radio Buttons**: Mutually exclusive options
- **Combo Box**: Dropdown selection
- **Color Picker**: RGBA color selection with live preview
- **Progress Bar**: Auto-incrementing progress indicator

### Calculator

- Full numeric keypad (0-9)
- Basic operations: +, -, \*, /
- Clear (C) and equals (=) functions
- Large, readable display
- Chained operations support

### Visualization

- **Line Plot**: Real-time data visualization using egui_plot
- **Color Preview**: Large display area showing selected color
- **Custom Drawing**: Interactive canvas with circles and mouse tracking

### UI Layout

- **Menu Bar**: File, View, and Help menus
- **Side Panels**: Organized left and right sidebars
- **Scrollable Areas**: Smooth scrolling for long content
- **Groups**: Visually organized widget sections
- **Grid Layout**: Calculator button grid

## Building and Running

```bash
cargo build --release
cargo run --release
```

## Why Egui?

### Advantages over Nuklear

- **Pure Rust**: No C dependencies, easier to build
- **Actively Maintained**: Regular updates and bug fixes
- **Modern API**: Clean, ergonomic Rust API
- **Cross-Platform**: Desktop, web (WASM), and mobile
- **Great Performance**: Efficient rendering
- **Rich Ecosystem**: Plots, markdown, syntax highlighting, etc.

### Advantages over ImGui

- **Pure Rust**: No C++ bindings needed
- **Web Support**: Compiles to WebAssembly
- **Simpler Build**: No external dependencies
- **Modern Design**: Built for Rust from the ground up

### Use Cases

- Game development tools
- Debug interfaces
- Data visualization
- Prototyping
- Desktop applications
- Web applications (via WASM)

## Code Structure

- `MyApp`: Main application state
- `update()`: Called every frame to build UI
- Calculator logic: Digit input and operations
- Plot updates: Rolling buffer visualization
- Custom painting: Direct drawing API

## Egui Features Demonstrated

1. **Panels**: Top, bottom, left, right, and central
2. **Layouts**: Horizontal, vertical, grid
3. **Widgets**: All standard UI controls
4. **Plotting**: Line charts with egui_plot
5. **Custom Drawing**: Painter API for custom graphics
6. **Menus**: Menu bar with dropdowns
7. **Scroll Areas**: Scrollable content regions
8. **Rich Text**: Sized, colored, and styled text
9. **Responsive**: Hover effects and interactions

## Extending the Example

Try adding:

- File dialogs (using rfd crate)
- Drag and drop
- Tabs and collapsing headers
- Tables with sorting
- Image display
- Markdown rendering (egui_extras)
- Syntax highlighting (egui_code_editor)
- 3D rendering integration
- Persistence (save/load state)

## Performance

Egui is designed for 60+ FPS interactive applications:

- Immediate mode: No state synchronization
- Efficient rendering: Only redraws when needed
- Small binary size: ~2-3 MB release builds
- Low memory usage: Minimal allocations per frame

## Resources

- [Egui Documentation](https://docs.rs/egui)
- [Egui GitHub](https://github.com/emilk/egui)
- [Online Demo](https://www.egui.rs/)
- [Egui Book](https://emilk.github.io/egui/)
