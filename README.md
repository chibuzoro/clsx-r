# clsx-r

A Rust macro utility for conditionally constructing strings, primarily used for CSS class names. Inspired by the JavaScript [clsx](https://github.com/lukeed/clsx) package.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clsx-r = "0.1.0"
```

## Usage

```rust
use clsx_r::clsx;

fn main() {
    // Basic usage
    let classes = clsx!("foo", "bar");  // => "foo bar"

    // With conditions
    let is_active = true;
    let is_disabled = false;
    let classes = clsx!(
        "btn",
        "primary",
        "active" => is_active,
        "disabled" => is_disabled
    );  // => "btn primary active"

    // With dynamic values
    let dynamic_class = "special";
    let classes = clsx!(
        "base",
        dynamic_class,
        "highlighted" => true
    );  // => "base special highlighted"
}
```

## Features

- Simple string concatenation
- Conditional class names
- Supports static and dynamic strings
- Filters out empty strings automatically
- Zero dependencies

## API

The `clsx!` macro accepts:

1. Simple string literals or expressions:
   ```rust
   clsx!("class1", "class2")
   ```

2. Conditional classes using the `=>` syntax:
   ```rust
   clsx!("class1" => condition, "class2" => other_condition)
   ```

3. Mixed usage:
   ```rust
   clsx!("static", dynamic_var, "conditional" => condition)
   ```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
