# globals_macro

A simple, zero-boilerplate solution for thread-safe global variables in Rust.

## Features

- **Effortless globals** - Define with a clean macro syntax
- **Thread-safe** - Built on `parking_lot`'s fast RwLock
- **Lazy initialization** - Uses `once_cell` for optimal performance
- **Ergonomic API** - Simple `.get()`, `.set()`, and `.update()` operations

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
globals_macro = { git = "https://github.com/Stuntlover-TM/globals-macro" }
```

## Basic Usage

```rust
use globals_macro::{globals, GlobalVar};

globals! {
    // Initialize with value
    app_name: String = "My App".to_string(),
    
    // Default-initialized (requires Default trait)
    user_count: usize,
    is_active: bool
}

fn main() {
    // Set values directly
    app_name.set("Awesome App".to_string());
    is_active.set(true);
    
    // Get cloned values
    println!("App: {}, Active: {}", app_name.get(), is_active.get());
    
    // Atomic update
    user_count.update(|c| *c += 1);  // Thread-safe increment
    
    // Efficient read without clone
    let name_len = app_name.get_with(|n| n.len());
    println!("Name length: {}", name_len);
}
```

## API Reference

### Macro Syntax
```rust
globals! {
    var_name: Type = initial_value,  // with initializer
    var_name: Type,                  // uses Type::default()
}                                     // (requires Default trait)
```

### Available Methods
- `.set(value)` - Updates the global value
- `.get() -> T` - Gets a cloned value (requires `T: Clone`)
- `.get_with(|val| ...)` - Operates on the value without cloning
- `.update(|mut val| ...)` - Atomically modifies the value

## Other example

```rust
use globals_macro::{globals, GlobalVar};

globals! {
    // Application configuration
    config: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("version".into(), "1.0".into());
        // Sets config to m by returning it
        m
    },
    
    // Request counter
    request_count: usize,
}

fn handle_request() {
    request_count.update(|c| *c += 1);
    
    config.get_with(|cfg| {
        println!("App version: {}", cfg.get("version").unwrap());
    });
}
```