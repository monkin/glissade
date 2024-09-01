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
    let animation = keyframes::line((0.0, 2.0), (1.0, 3.0), 1.0).run(0.0f32);
    print_1s_values(animation);

    println!("\nInertial:");
    let inertial = Inertial::new(5.0).go_to(10.0, 0.0, 1.0);
    print_1s_values(inertial);

    println!("\nStationary:");
    let stationary = 42;
    print_1s_values(stationary);

    println!("\nMapped animation:");
    let animation = keyframes::from((0.0, 0.0))
        .go_to((100.0, 40.0), 1.0)
        .run(0.0)
        .map(|v| format!("left: {:.2}; top: {:.2};", v.0, v.1));
    print_1s_values(animation);
}
