# Understanding the Plugin System

Nebula2D uses a plugin-based architecture to provide modularity and extensibility.

## Plugin Basics

A plugin in Nebula2D is any type that implements the `Plugin` trait:

```rust
pub trait Plugin {
    fn build(&self, app: &mut App);
}
```

## Creating a Custom Plugin

Here's how to create a simple plugin:

```rust
use nebula2d::prelude::*;

pub struct MyPlugin {
    // Your plugin's data
}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        // Initialize your plugin
        // Configure the app
        // Set up resources
    }
}
```

## Using Plugins

Plugins are added to your app using the `add_plugin` method:

```rust
let mut app = App::new();

// Add single plugin
app.add_plugin(MyPlugin::new());

// Chain multiple plugins
app.add_plugin(WindowPlugin::new())
   .add_plugin(MyPlugin::new())
   .build();
```

## Plugin Best Practices

1. **State Management**
   ```rust
   pub struct MyPluginState {
       // Plugin-specific state
   }

   pub struct MyPlugin {
       state: MyPluginState,
   }
   ```

2. **Event Handling**
   ```rust
   impl MyPlugin {
       pub fn handle_event(&mut self, event: Event) {
           // Handle events specific to your plugin
       }
   }
   ```

3. **Configuration**
   ```rust
   pub struct MyPluginConfig {
       // Configuration options
   }

   impl MyPlugin {
       pub fn with_config(config: MyPluginConfig) -> Self {
           // Create plugin with custom config
       }
   }
   ```

## Example: Creating a Simple Plugin

Here's a complete example of a custom plugin:

```rust
use nebula2d::prelude::*;

// Plugin configuration
pub struct CounterConfig {
    initial_value: i32,
}

// Plugin state
pub struct CounterState {
    count: i32,
}

// The plugin
pub struct CounterPlugin {
    config: CounterConfig,
    state: CounterState,
}

impl CounterPlugin {
    pub fn new(initial_value: i32) -> Self {
        Self {
            config: CounterConfig { initial_value },
            state: CounterState { count: initial_value },
        }
    }
}

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        // Initialize plugin in the app
        println!("Counter initialized at: {}", self.state.count);
    }
}

// Usage
fn main() {
    let mut app = App::new();
    app.add_plugin(CounterPlugin::new(42))
        .build();
}
```

## Plugin Architecture Tips

1. Keep plugins focused on a single responsibility
2. Use appropriate state management
3. Provide configuration options
4. Handle cleanup when necessary
5. Document plugin functionality

## Built-in Plugins

Currently available plugins:
- `WindowPlugin`: Window management
- More coming soon! 