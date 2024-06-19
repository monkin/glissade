# Glissade

Glissade is a Rust library that provides various utilities for animations and transitions.

## Features

- InertialValue: A value that changes over time towards a target value.
- Transition: Represents transitioning between two states.
- Animation: Provides functionality for animating values (running Transition,
  or in other words a Transition with start time attached).
- Easing: Contains various easing functions for smooth transitions.
- Mix: A trait for mixing two values with a certain ratio.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
glissade = "0.1.0"
```

And this to your crate root:

```rust
extern crate glissade;
```

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
