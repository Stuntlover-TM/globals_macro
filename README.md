# globals_macro

A simple, zero-boilerplate solution for **thread-safe global variables** in Rust, powered by `parking_lot` and `once_cell`.

## Features

- **Two macro types**:
  - `globals!`: Thread-safe mutable globals (RwLock-protected)
  - `const_globals!`: Immutable lazy-initialized values (just `Lazy<T>`)
- **Optimized access patterns**:
  - Clone-free reads with `get_with()`
  - Atomic updates for mutable globals
- **Thread-safe by design**:
  - Uses `parking_lot`'s optimized `RwLock` for mutable globals
  - Lock-free access for constants
- **Lazy initialization** - Values created on first access via `once_cell`

## Installation

```toml
[dependencies]
globals_macro = { git = "https://github.com/Stuntlover-TM/globals_macro" }
```

## When to Use Which Macro

| Macro | Best For | Thread Safety | Initialization |
|-------|----------|---------------|----------------|
| `const_globals!` | Truly immutable values, configuration | Lock-free | First access |
| `globals!` | Mutable state, shared resources | RwLock-protected | First access |

## Basic Usage

```rust
use globals_macro::*;
use std::collections::HashMap;

// Immutable constants
const_globals! {
    VERSION: &'static str = "1.0",
    MAX_USERS: usize = 100,
}

// Mutable state
globals! {
    active_users: usize,
    app_config: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("timeout".into(), "30s".into());
        m
    },
}

fn main() {
    // Access constants (no locks needed)
    println!("v{} (max users: {})", VERSION.get(), MAX_USERS.get());
    
    // Modify state (thread-safe)
    active_users.update(|count| *count += 1);
    
    // Efficient read without clone
    let timeout = app_config.get_with(|c| c.get("timeout").unwrap().clone());
    println!("Timeout: {}", timeout);
    
    // Update a Map
    app_config.update(|c| { c.insert("timeout".to_string(), "60s".to_string()); });

    // Full replacement
    active_users.set(0);
}
```

## Threading Behavior

### For `globals!` (Mutable):

| Operation  | Behavior | Blocks? | Safe? |
|------------|----------|---------|-------|
| `.get()`   | Multiple readers | ❌ | ✅ |
| `.get_with()` | Multiple readers | ❌ | ✅ |
| `.set()`   | Exclusive write | ✅ | ✅ |
| `.update()` | Exclusive write | ✅ | ✅ |

### For `const_globals!` (Immutable):

| Operation  | Behavior | Blocks? | Safe? |
|------------|----------|---------|-------|
| `.get()`   | Lock-free read | ❌ | ✅ |
| `.get_with()` | Lock-free read | ❌ | ✅ |

**Key Rules:**
1. Multiple reads can happen concurrently for both types
2. Writes block all other access to that specific global
3. No deadlocks possible with `const_globals!`

## Advanced Patterns

### Configuration Example
```rust
const_globals! {
    DEFAULT_CONFIG: Config = Config {
        timeout: 30,
        retries: 3,
    },
}

// Later in tests
#[test]
fn test_config() {
    assert_eq!(DEFAULT_CONFIG.get().retries, 3);
}
```

## API Reference

### Macro Syntax
```rust
const_globals! {
    NAME: Type = initializer,  // Required initializer
}

globals! {
    NAME: Type = initializer,  // Optional initializer (defaults to Type::default())
}
```

### Common Methods
| Method | Works On | Description |
|--------|----------|-------------|
| `.get() -> T` | Both | Returns cloned value (requires `T: Clone`) |
| `.get_with(\|val\| ...)` | Both | Reference access without clone |

### Mutable-Only Methods
| Method | Description |
|--------|-------------|
| `.set(value)` | Atomic replacement |
| `.update(\|mut val\| ...)` | In-place modification |

## Performance Tips

1. **Prefer `const_globals!`** for truly immutable data
2. **Use `get_with()`** to avoid clone overhead
3. **Group updates** to minimize lock time:
   ```rust
   // Good:
   config.update(|c| {
       c.timeout = 60;
       c.retries = 5;
   });
   
   // Bad:
   config.update(|c| c.timeout = 60);
   config.update(|c| c.retries = 5);
   ```