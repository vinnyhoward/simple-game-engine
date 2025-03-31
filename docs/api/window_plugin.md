# Window Plugin API Reference

The Window Plugin provides window management functionality for Nebula2D applications.

## Structures

### WindowPlugin

Main plugin structure for window management.

```rust
pub struct WindowPlugin {
    // ... internal fields ...
}
```

#### Methods

##### `new() -> Self`
Creates a new WindowPlugin with default settings.

```rust
let plugin = WindowPlugin::new();
```

##### `with_attributes(window_attributes: WindowAttributes) -> Self`
Creates a new WindowPlugin with custom window attributes.

```rust
let mut attributes = Window::default_attributes();
attributes.title = "Custom Window".into();
let plugin = WindowPlugin::with_attributes(attributes);
```

### WindowConfig

Configuration structure for window settings.

```rust
pub struct WindowConfig {
    pub attributes: WindowAttributes,
}
```

### WindowState

Manages the window state.

```rust
pub struct WindowState {
    pub window: Option<Window>,
    pub attributes: WindowAttributes,
}
```

## Events

The window plugin handles the following events:
- `WindowEvent::CloseRequested`: Handles window close button clicks
- `WindowEvent::RedrawRequested`: Manages window redraw requests

## Usage Examples

### Basic Window

```rust
use nebula2d::prelude::*;

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    
    // Add default window
    app.add_plugin(WindowPlugin::new())
        .build();

    event_loop.run_app(&mut app)?;
    Ok(())
}
```

### Customized Window

```rust
let mut attributes = Window::default_attributes();
attributes.title = "Custom Window".into();
attributes.inner_size = Some(Size::Physical(PhysicalSize::new(1024, 768)));

app.add_plugin(WindowPlugin::with_attributes(attributes))
    .build();
```

## Implementation Details

The window plugin:
1. Stores window configuration in the App
2. Manages window state
3. Handles window events
4. Creates and maintains the window instance

## Future Enhancements

Planned features:
- Multiple window support
- Additional window events
- Window positioning
- Fullscreen toggle
- More customization options 