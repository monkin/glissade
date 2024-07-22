use color::Color;
use glissade::Inertial;

mod color;

fn main() {
    let mut color = Inertial::new(Color::red());

    println!("Static color for one second.");
    for time in [0.0, 0.25, 0.5, 0.75, 1.0].iter().copied() {
        println!("{} at {:.2}s", color.get(time), time);
    }

    println!("\nThen go to green in 2 seconds.");
    color = color.go_to(Color::green(), 1.0, 2.0);
    for time in [1.25, 1.5, 1.75, 2.0].iter().copied() {
        println!("{} at {:.2}s", color.get(time), time);
    }

    println!("\nIn the middle of the transition change direction to blue.");
    color = color.go_to(Color::blue(), 2.0, 2.0);
    for time in [2.25, 2.5, 2.75, 3.0, 3.25, 3.5, 3.75, 4.0, 4.25, 4.5]
        .iter()
        .copied()
    {
        println!("{} at {:.2}s", color.get(time), time);
    }
}
