# Glissade

[![Tests Status](https://github.com/monkin/glissade/actions/workflows/tests.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/tests.yml)
[![Build Status](https://github.com/monkin/glissade/actions/workflows/build.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/build.yml)
[![Clippy Status](https://github.com/monkin/glissade/actions/workflows/clippy.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/clippy.yml)
[![Format Status](https://github.com/monkin/glissade/actions/workflows/format.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/format.yml)
[![Pages Status](https://github.com/monkin/glissade/actions/workflows/pages.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/pages.yml)

Glissade is a Rust animations and transitions library. It's framework-agnostic with optional
[euclid](https://crates.io/crates/euclid), [nalgebra](https://crates.io/crates/nalgebra),
[cgmath](https://crates.io/crates/cgmath), [glam](https://crates.io/crates/glam),
and [palette](https://crates.io/crates/palette) support.
To make it work, you need to enable the corresponding feature.

The lib contains two main types: `Animation` and `Inertial`.
* `Animation` contains `Keyframes` and can be used in cases when we know start, end, and in between points. It's similar to [CSS animations/keyframes](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_animations/Using_CSS_animations).
* `Inertial` can be used to make an object smoothly follow a target value. It's similar to [CSS transitions](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_transitions/Using_CSS_transitions).
  For example, a particle following a cursor. Background color changing smoothly on theme change.

It also contains a set of easing functions to make animations more natural. See the `Easing` enum for more details.

To make code more general the library contains `Animated` trait which is implemented for both `Animation` and `Inertial`.
With `Stationary` trait it's easy to pass static value in places where `Animated` expected.
By default, it's implemented for primitive types like numbers or strings.

Most of the methods receive time as a parameter to allow testing without mocks,
and have a consistent behavior during a single animation frame. It's expected that time is received, for example,
from `Instant::now()` once at the beginning of the frame, and used lately during the frame rendering.

Any type that implements `Time` trait can be used as a time type. By default, it's implemented for `std::time::Instant`,
`std::time::SystemTime`, f32, and f64. It's also implemented for `web_time::*` if `"web-time"` feature is enabled.
It's recommended to use `web_time::Instant` and `web_time::Duration` as a time type in most cases.

Animation can be applied to any type that implements `Mix` trait. This trait is used to interpolate between two values.
Mix trait is implemented for common types like `f32`, `f64`, `bool`, `i8` - `i64`, `u8` - `u64`, `Option<T: Mix>`,
and tuples like `(Mix, Mix)`, `(Mix, Mix, Mix)`, etc. It's also implemented for some popular libraries:
[`nalgebra`](https://crates.io/crates/nalgebra), [`euclid`](https://crates.io/crates/euclid),
[`cgmath`](https://crates.io/crates/cgmath), [`glam`](https://crates.io/crates/glam), and [`palette`](https://crates.io/crates/palette).

The full documentation is available on [docs.rs](https://docs.rs/glissade).

## Derive macro

The library contains a derive macro to implement the `Mix` trait for structs and tuples.

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

## Cargo features

* `"derive"` - enables derive macro for `Mix` trait. Enabled by default.
* `"euclid"` - enables [euclid](https://crates.io/crates/euclid) vectors, rotations, etc. animation.
* `"nalgebra"` - enables [nalgebra](https://crates.io/crates/nalgebra) vectors, matrices, transformations, etc. animation.
* `"cgmath"` - enables [cgmath](https://crates.io/crates/cgmath) vectors, matrices, etc. animation.
* `"glam"` - enables [glam](https://crates.io/crates/glam) vectors, matrices, etc. animation.
* `"palette"` - enables [palette](https://crates.io/crates/palette) colors interpolation.
* `"web-time"` - use `web_time::*` instead of `std::time::*` for `Instant` and `Duration` types. It doesn't change
  anything for desktop platforms, but allows to use the same code for WASM. Enabled by default.

## Examples

### Live

* Following a path using keyframes [[Live](https://monkin.github.io/glissade/poly/)] [[Source](https://github.com/monkin/glissade/tree/master/examples/poly)]
* Animating a shape using Inertial [[Live](https://monkin.github.io/glissade/shape-animation/)] [[Source](https://github.com/monkin/glissade/tree/master/examples/shape-animation)]
* A set of particles following the cursor made with Inertial [[Live](https://monkin.github.io/glissade/follow-cursor/)] [[Source](https://github.com/monkin/glissade/tree/master/examples/follow-cursor)]

### Simple two-step `Animation`

```rust
use glissade::{keyframes, Animated, Easing, Keyframes};
use std::thread::sleep;
use std::time::{Duration, Instant};

const STEPS_COUNT: u32 = 10;
const STEP: Duration = Duration::from_millis(3500 / STEPS_COUNT as u64);

fn main() {
    let start_time = Instant::now();

    // Transition consists of two steps:
    // 1. from 0.0 to 10.0 in 1 second linearly,
    // 2. and then go to 5.0 with easing function.
    let animation = keyframes(0.0)
        .go_to(10.0, Duration::from_secs(1))
        .ease_to(5.0, Duration::from_secs(2), Easing::QuadraticInOut)
        .run(start_time);

    for _ in 0..STEPS_COUNT {
        println!(
            "{:.2}s: {:.4}",
            start_time.elapsed().as_secs_f64(),
            animation.get(Instant::now())
        );
        sleep(STEP);
    }
}
```

It prints the following output:
```text
0.00s: 0.0000
0.35s: 3.5000
0.70s: 7.0000
1.05s: 9.9935
1.40s: 9.5980
1.75s: 8.5862
2.10s: 7.0160
2.45s: 5.7480
2.80s: 5.0970
3.15s: 5.0000
```

Try it yourself with `cargo run -p console-transition` or view the source code in [./examples/console-transition](https://github.com/monkin/glissade/tree/master/examples/console-transition).

### Smoothly change color using `Inertial`

```rust
use glissade::{Animated, Inertial};

type Color = (u8, u8, u8);

const RED: Color = (255, 0, 0);
const GREEN: Color = (0, 255, 0);
const BLUE: Color = (0, 0, 255);

fn main() {
    let mut color = Inertial::new(RED);

    println!("Static color for one second.");
    for time in [0.0, 0.25, 0.5, 0.75, 1.0].iter().copied() {
        println!("{:.2}s: {:?}", time, color.get(time));
    }

    println!("\nThen go to green in 2 seconds.");
    color = color.go_to(GREEN, 1.0, 2.0);
    for time in [1.25, 1.5, 1.75, 2.0].iter().copied() {
        println!("{:.2}s: {:?}", time, color.get(time));
    }

    println!("\nIn the middle of the transition change direction to blue.");
    color = color.go_to(BLUE, 2.0, 2.0);
    for time in [2.25, 2.5, 2.75, 3.0, 3.25, 3.5, 3.75, 4.0, 4.25, 4.5]
        .iter()
        .copied()
    {
        println!("{:.2}s: {:?}", time, color.get(time));
    }
}
```

It prints the following output:
```text
Static color for one second.
0.00s: (255, 0, 0)
0.25s: (255, 0, 0)
0.50s: (255, 0, 0)
0.75s: (255, 0, 0)
1.00s: (255, 0, 0)

Then go to green in 2 seconds.
1.25s: (247, 8, 0)
1.50s: (223, 32, 0)
1.75s: (183, 72, 0)
2.00s: (128, 128, 0)

In the middle of the transition change direction to blue.
2.25s: (70, 177, 8)
2.50s: (28, 195, 32)
2.75s: (6, 178, 72)
3.00s: (0, 128, 128)
3.25s: (0, 72, 183)
3.50s: (0, 32, 223)
3.75s: (0, 8, 247)
4.00s: (0, 0, 255)
4.25s: (0, 0, 255)
4.50s: (0, 0, 255)
```

Try it yourself with `cargo run -p console-inertial` or view the source code in [./examples/console-inertial](https://github.com/monkin/glissade/tree/master/examples/console-transition).

### Use the same code for `Animation`, `Inertial`, and `Stationary`

```rust
use glissade::{keyframes, Animated, Inertial, Keyframes};
use std::fmt::Debug;

/// Print the values of an animated value at 0.0, 0.25, 0.5, 0.75, and 1.0.
/// Any value that implements `Animated` can be passed to this function.
/// So, it can accept animations, inertial, and stationary values.
pub fn print_1s_values<T: Clone + Debug>(value: impl Animated<T, f32>) {
    for i in [0.0, 0.25, 0.5, 0.75, 1.0].iter() {
        println!("{:.2}s: {:?}", i, value.get(*i));
    }
}

fn main() {
    println!("Animation:");
    let animation = keyframes((0.0, 2.0)).go_to((1.0, 3.0), 1.0).run(0.0);
    print_1s_values(animation);

    println!("\nInertial:");
    let inertial = Inertial::new(5.0).go_to(10.0, 0.0, 1.0);
    print_1s_values(inertial);

    println!("\nStationary:");
    let stationary = 42;
    print_1s_values(stationary);

    println!("\nMapped animation:");
    let animation = keyframes((0.0, 0.0))
        .go_to((100.0, 40.0), 1.0)
        .run(0.0)
        .map(|v| format!("left: {:.2}; top: {:.2};", v.0, v.1));
    print_1s_values(animation);
}
```

It prints the following output:
```text
Animation:
0.00s: (0.0, 2.0)
0.25s: (0.25, 2.25)
0.50s: (0.5, 2.5)
0.75s: (0.75, 2.75)
1.00s: (1.0, 3.0)

Inertial:
0.00s: 5.0
0.25s: 5.625
0.50s: 7.5
0.75s: 9.375
1.00s: 10.0

Stationary:
0.00s: 42
0.25s: 42
0.50s: 42
0.75s: 42
1.00s: 42

Mapped animation:
0.00s: "left: 0.00; top: 0.00;"
0.25s: "left: 25.00; top: 10.00;"
0.50s: "left: 50.00; top: 20.00;"
0.75s: "left: 75.00; top: 30.00;"
1.00s: "left: 100.00; top: 40.00;"
```

Try it yourself with `cargo run -p animated` or view the source code in [./examples/animated](https://github.com/monkin/glissade/tree/master/examples/animated).

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/monkin/glissade/blob/master/LICENSE.md) file for details.
