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
