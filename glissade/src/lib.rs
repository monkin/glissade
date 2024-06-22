//! # A library for creating animations and transitions in Rust.
//!
//! The lib contains two main types: [`Animation`] and [`InertialValue`].
//! * [`Animation`] can be used in cases when we know start, end, and in between keyframes.
//! * [`InertialValue`] can be used to make an object smoothly follow a target value.
//!   For example, a particle following a cursor. Background color changing smoothly on theme change.
//!
//! It also contains a set of easing functions to make animations more natural. See the [`Easing`] enum for more details.
//!
//! Animation can be applied to any type that implements [`Mix`] trait. This trait is used to interpolate between two values.
//! Mix trait is implemented for common types like `f32`, `f64`, `bool`, `i8` - `i64`, `u8` - `u64`, `Option<T: Mix>`,
//! and tuples like `(Mix, Mix)`, `(Mix, Mix, Mix)`, etc. It's also implemented for some popular libraries:
//! [`nalgebra`](https://crates.io/crates/nalgebra), [`euclid`](https://crates.io/crates/euclid), and
//! [`palette`](https://crates.io/crates/palette). To make it work, you need to enable the corresponding feature.
//!
//! Most of the methods receive `SystemTime` as a parameter to allow testing without mocks,
//! and have a consistent behavior during a single animation frame. It's expected that time is received
//! from `SystemTime::now()` once in the beginning of the frame, and used lately during the frame rendering.
//!
//! # Examples
//!
//! ## Simple two-step animation
//!
//! ```
//! use glissade::{Easing, transition, Transition};
//! use std::time::{Duration, SystemTime};
//!
//! // Create an animation template - a transition.
//! //
//! // This transition consists of two steps:
//! // 1. from 0.0 to 10.0 in 1 second linearly,
//! // 2. and then go to 5.0 with easing function.
//! let transition = transition(0.0)
//!     .linear_to(10.0, Duration::from_secs(1))
//!     .ease_to(5.0, Duration::from_secs(2), Easing::QuadraticInOut);
//!
//! let now = SystemTime::now();
//! // Create an animation from the transition and start time.
//! let animation = transition.run(now);
//!
//! assert_eq!(animation.get(now), 0.0);
//! assert_eq!(animation.get(now + Duration::from_millis(500)), 5.0);
//! assert_eq!(animation.get(now + Duration::from_secs(1)), 10.0);
//! assert_eq!(animation.get(now + Duration::from_secs(2)), 7.5);
//! assert_eq!(animation.get(now + Duration::from_secs(3)), 5.0);
//!```
//!
//! ## Smoothly change color
//!
//!```
//! use glissade::{InertialValue, Easing};
//! use std::time::{Duration, SystemTime};
//!
//! type Color = (f32, f32, f32);
//!
//! let start_time = SystemTime::now();
//!
//! // Create initial black value
//! let value: InertialValue<Color> = InertialValue::new((0.0, 0.0, 0.0), start_time);
//!
//! assert_eq!(value.get(start_time), (0.0, 0.0, 0.0));
//! assert_eq!(value.get(start_time + Duration::from_secs(1)), (0.0, 0.0, 0.0));
//!
//! // Change color to white in one second
//! let value = value.go_to((1.0, 1.0, 1.0), start_time, Duration::from_secs(1));
//!
//! assert_eq!(value.get(start_time), (0.0, 0.0, 0.0));
//! assert_eq!(value.get(start_time + Duration::from_millis(500)), (0.5, 0.5, 0.5));
//! assert_eq!(value.get(start_time + Duration::from_secs(1)), (1.0, 1.0, 1.0));
//! assert_eq!(value.get(start_time + Duration::from_secs(2)), (1.0, 1.0, 1.0));
//!
//! // Change color to red in between the transition
//! let value = value.ease_to((1.0, 0.0, 0.0), start_time + Duration::from_millis(500), Duration::from_secs(2), Easing::Linear);
//!
//! assert_eq!(value.get(start_time + Duration::from_millis(500)), (0.5, 0.5, 0.5));
//! assert_eq!(value.get(start_time + Duration::from_secs(1)), (1.0, 0.75, 0.75));
//! assert_eq!(value.get(start_time + Duration::from_secs(2)), (1.0, 0.25, 0.25));
//! assert_eq!(value.get(start_time + Duration::from_millis(2500)), (1.0, 0.0, 0.0));
//! assert_eq!(value.get(start_time + Duration::from_secs(4)), (1.0, 0.0, 0.0));
//! ```

mod animation;
mod easing;
mod inertial_value;
mod mix;
mod transition;
mod transition_item;

#[cfg(feature = "euclid")]
mod euclid;
#[cfg(feature = "nalgebra")]
mod nalgebra;
#[cfg(feature = "palette")]
mod palette;

pub use animation::Animation;
pub use easing::Easing;
pub use inertial_value::InertialValue;
pub use mix::Mix;
pub use transition::{transition, Transition};
