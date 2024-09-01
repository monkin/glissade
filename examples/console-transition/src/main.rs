//! Two-step transition with easing function example.

use glissade::{keyframes, Animated, Easing, Keyframes};
use std::thread::sleep;
use std::time::{Duration, Instant};

const STEPS_COUNT: u32 = 10;
const STEP: Duration = Duration::from_millis(3500 / STEPS_COUNT as u64);

fn main() {
    let start_time = Instant::now();

    // Transition consists of two steps:
    // 1. From 0.0 to 10.0 in 1 second linearly,
    // 2. And then go to 5.0 with easing function.
    let animation = keyframes::from(0.0)
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
