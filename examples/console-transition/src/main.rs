//! Two-step transition with easing function example.

use glissade::{transition, Easing, Transition};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const STEPS_COUNT: u32 = 10;
const STEP: Duration = Duration::from_millis(3500 / STEPS_COUNT as u64);

fn main() {
    // Transition consists of two steps:
    // 1. from 0.0 to 10.0 in 1 second linearly,
    // 2. and then go to 5.0 with easing function.
    let animation = transition(0.0)
        .go_to(10.0, Duration::from_secs(1))
        .ease_to(5.0, Duration::from_secs(2), Easing::QuadraticInOut)
        .run(SystemTime::now());

    for i in 0..STEPS_COUNT {
        println!(
            "{:.2}s: {:.4}",
            (STEP * i).as_secs_f64(),
            animation.get(SystemTime::now())
        );
        sleep(STEP);
    }
}
