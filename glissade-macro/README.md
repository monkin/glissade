# Glissade macro

Glissade macro is a Rust crate that provides `Mix` derive macro
for the [`glissade`](https://github.com/monkin/glissade) library.

See the https://github.com/monkin/glissade for more information.

## Usage
```rust
use glissade::Mix;

#[derive(Mix, PartialEq, Debug)]
struct Touch {
    x: f32,
    y: f32,
    pressure: u8,
}

let touch1 = Touch { x: 0.0, y: 0.0, pressure: 0 };
let touch2 = Touch { x: 100.0, y: 100.0, pressure: 200 };
let touch_mix = touch1.mix(touch2, 0.5);

assert_eq!(touch_mix, Touch { x: 50.0, y: 50.0, pressure: 100 });
```

## License

This project is licensed under the MIT License.