# globals_macro

A simple, zero-boilerplate solution for **thread-safe global variables** in Rust, powered by `parking_lot` and `once_cell`.

## Features

- **Effortless declaration** - Clean macro syntax for defining globals
- **Thread-safe by design** - Uses `parking_lot`'s optimized `RwLock`
- **Lazy initialization** - Values created on first access via `once_cell`
- **Flexible access** - Choose between cloning or reference-based access
- **Writer priority** - Fair lock acquisition prevents writer starvation

## Installation

```toml
[dependencies]
globals_macro = { git = "https://github.com/Stuntlover-TM/globals-macro" }
```

## Threading Behavior

| Operation  | Concurrent Access Behavior              | Blocks? | Safe? |
|------------|----------------------------------------|---------|-------|
| `.get()`   | Multiple readers allowed                | ❌      | ✅    |
| `.get_with()` | Multiple readers allowed            | ❌      | ✅    |
| `.set()`   | Exclusive access; blocks all readers/writers | ✅ (waits) | ✅    |
| `.update()` | Exclusive access; blocks all readers/writers | ✅ (waits) | ✅    |

### Key Threading Rules:
1. **Reads (`.get()`, `.get_with()`)**  
   - Can run concurrently with other reads.  
   - Blocked only during active writes.

2. **Writes (`.set()`, `.update()`)**  
   - Require exclusive access.  
   - Block until all existing reads/writes complete.  
   - New reads/writes wait until the write finishes.

3. **Deadlock Warning**  
   ```rust
   global.update(|x| {
       global.set(42); // ⚠️ Deadlock! (nested write)
   });
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
    // Set values
    app_name.set("Awesome App".to_string());
    
    // Thread-safe read (clones)
    println!("App: {}", app_name.get());
    
    // Atomic update
    user_count.update(|c| *c += 1);  // Thread-safe increment
    
    // Efficient read without clone
    let _len = app_name.get_with(|n| n.len());
}
```

### Map Example
```rust
globals! {
    config: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("timeout".into(), "30s".into());
        m
    },
}

fn request() {
    let timeout = config.get_with(|c| c.get("timeout").unwrap());
    // ...
}
```

## API Reference

### Macro Syntax
```rust
globals! {
    var_name: Type = initializer,  // With explicit init
    var_name: Type,               // Uses Type::default()
}
```

### Methods
| Method | Description | Clone Required? |
|--------|-------------|-----------------|
| `.set(value)` | Replaces the value | ❌ |
| `.get() -> T` | Returns a cloned value | ✅ |
| `.get_with(\|val\| ...)` | Reads without clone | ❌ |
| `.update(\|mut val\| ...)` | Modifies in-place | ❌ |