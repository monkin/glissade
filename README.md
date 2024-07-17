# Glissade

[![Tests Status](https://github.com/monkin/glissade/actions/workflows/tests.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/tests.yml)
[![Build Status](https://github.com/monkin/glissade/actions/workflows/build.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/build.yml)
[![Clippy Status](https://github.com/monkin/glissade/actions/workflows/clippy.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/clippy.yml)
[![Format Status](https://github.com/monkin/glissade/actions/workflows/format.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/format.yml)
[![Pages Status](https://github.com/monkin/glissade/actions/workflows/pages.yml/badge.svg)](https://github.com/monkin/glissade/actions/workflows/pages.yml)

Glissade is a Rust animations and transitions library. It's framework-agnostic with optional
[euclid](https://crates.io/crates/euclid), [nalgebra](https://crates.io/crates/nalgebra),
[cgmath](https://crates.io/crates/cgmath), and [palette](https://crates.io/crates/palette) support.
To make it work, you need to enable the corresponding feature.

The lib contains two main types: `Animation` and `InertialValue`.
* `Animation` can be used in cases when we know start, end, and in between keyframes.
* `InertialValue` can be used to make an object smoothly follow a target value.
  For example, a particle following a cursor. Background color changing smoothly on theme change.

It also contains a set of easing functions to make animations more natural. See the `Easing` enum for more details.

Most of the methods receive `SystemTime` as a parameter to allow testing without mocks,
and have a consistent behavior during a single animation frame. It's expected that time is received
from `SystemTime::now()` once in the beginning of the frame, and used lately during the frame rendering.

Animation can be applied to any type that implements `Mix` trait. This trait is used to interpolate between two values.
Mix trait is implemented for common types like `f32`, `f64`, `bool`, `i8` - `i64`, `u8` - `u64`, `Option<T: Mix>`,
and tuples like `(Mix, Mix)`, `(Mix, Mix, Mix)`, etc. It's also implemented for some popular libraries:
[`nalgebra`](https://crates.io/crates/nalgebra), [`euclid`](https://crates.io/crates/euclid),
[`cgmath`](https://crates.io/crates/cgmath), and [`palette`](https://crates.io/crates/palette).

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

* `"derive"` - enables derive macro for `Mix` trait (enabled by default).
* `"euclid"` - enables [euclid](https://crates.io/crates/euclid) vectors, rotations, etc. animation.
* `"nalgebra"` - enables [nalgebra](https://crates.io/crates/nalgebra) vectors, matrices, transformations, etc. animation.
* `"cgmath"` - enables [cgmath](https://crates.io/crates/cgmath) vectors, matrices, etc. animation.
* `"palette"` - enables [palette](https://crates.io/crates/palette) colors interpolation.

## Examples

### Live

* InertialValue [[Live](https://monkin.github.io/glissade/inertial/)] [[Source](https://github.com/monkin/glissade/tree/master/examples/inertial)]

### Simple two-step animation

```rust
//! Two-step transition with easing function example.

use glissade::{transition, Easing, Transition};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const STEPS_COUNT: u32 = 10;
const STEP: Duration = Duration::from_millis(3500 / STEPS_COUNT as u64);

fn main() {
    let start_time = SystemTime::now();

    // Transition consists of two steps:
    // 1. from 0.0 to 10.0 in 1 second linearly,
    // 2. and then go to 5.0 with easing function.
    let animation = transition(0.0)
        .go_to(10.0, Duration::from_secs(1))
        .ease_to(5.0, Duration::from_secs(2), Easing::QuadraticInOut)
        .run(start_time);

    for _ in 0..STEPS_COUNT {
        println!(
            "{:.2}s: {:.4}",
            start_time.elapsed().unwrap().as_secs_f64(),
            animation.get(SystemTime::now())
        );
        sleep(STEP);
    }
}
```

Prints the following output:
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

Try it yourself with `cargo run examples/console-transition`, or view the source code in [./examples/console-transition](https://github.com/monkin/glissade/tree/master/examples/console-transition).

### Smoothly change color

```rust
use glissade::{InertialValue, Easing};
use web_time::{Duration, SystemTime};

type Color = (f32, f32, f32);

let start_time = SystemTime::now();

// Create initial black value
let value: InertialValue<Color> = InertialValue::new((0.0, 0.0, 0.0));

assert_eq!(value.get(start_time), (0.0, 0.0, 0.0));
assert_eq!(value.get(start_time + Duration::from_secs(1)), (0.0, 0.0, 0.0));

// Change color to white in one second
let value = value.go_to((1.0, 1.0, 1.0), start_time, Duration::from_secs(1));

assert_eq!(value.get(start_time), (0.0, 0.0, 0.0));
assert_eq!(value.get(start_time + Duration::from_millis(500)), (0.5, 0.5, 0.5));
assert_eq!(value.get(start_time + Duration::from_secs(1)), (1.0, 1.0, 1.0));
assert_eq!(value.get(start_time + Duration::from_secs(2)), (1.0, 1.0, 1.0));

// Change color to red in between the transition
let value = value.ease_to((1.0, 0.0, 0.0), start_time + Duration::from_millis(500), Duration::from_secs(2), Easing::Linear);

assert_eq!(value.get(start_time + Duration::from_millis(500)), (0.5, 0.5, 0.5));
assert_eq!(value.get(start_time + Duration::from_secs(1)), (1.0, 0.75, 0.75));
assert_eq!(value.get(start_time + Duration::from_secs(2)), (1.0, 0.25, 0.25));
assert_eq!(value.get(start_time + Duration::from_millis(2500)), (1.0, 0.0, 0.0));
assert_eq!(value.get(start_time + Duration::from_secs(4)), (1.0, 0.0, 0.0));
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/monkin/glissade/blob/master/LICENSE.md) file for details.
