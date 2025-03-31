# Nebula2D Game Engine

A lightweight, modular 2D game engine built in Rust, inspired by Bevy's plugin architecture. Nebula2D focuses on simplicity and extensibility, making it ideal for 2D game development while providing insights into game engine architecture.

## Features

Current:
- 🔌 Plugin-based architecture for modular functionality
- 🪟 Configurable window management system
- 🎮 Event handling system

Planned:
- 🎨 2D rendering with wgpu
- 🖼️ Sprite and animation support
- ⌨️ Input handling system
- 🔊 Audio system
- 📦 Asset management
- 💫 Basic physics and collision detection

## Project Structure

```
src/
├── core/               # Core engine functionality
│   ├── app.rs         # Main app structure
│   └── plugin.rs      # Plugin trait definition
│
├── plugins/           # Engine plugins
│   └── window/       # Window management
│       ├── plugin.rs # Window plugin implementation
│       ├── state.rs  # Window state management
│       └── events.rs # Window event handling
│
└── main.rs           # Entry point
```

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo

### Building
```bash
cargo build
```

### Running the Example
```bash
cargo run
```

## Usage Example

```rust
use nebula2d::prelude::*;

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    
    // Configure window
    let mut attributes = Window::default_attributes();
    attributes.title = "My Game".into();
    
    // Initialize app with plugins
    let mut app = App::new();
    app.add_plugin(WindowPlugin::with_attributes(attributes))
        .build();

    event_loop.run_app(&mut app)?;
    Ok(())
}
```

## Design Philosophy

Nebula2D is built with the following principles:
- **Modularity**: Plugin-based architecture for extensible functionality
- **Learning-Focused**: Clear, well-documented code for understanding game engine concepts
- **Performance**: Efficient, low-level implementations where possible
- **Simplicity**: Focus on essential features for 2D game development

## Contributing

This project is currently in development. Contributions and suggestions are welcome!

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by the Bevy game engine's plugin architecture
- Built with Rust and winit

## Current Status

🚧 **In Development** - Basic architecture and window management implemented. Rendering system coming soon! 