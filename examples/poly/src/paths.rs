use glam::Vec2;
use std::f32::consts::PI;

pub fn ngon(size: f32, n: usize) -> impl Iterator<Item = Vec2> {
    (0..=n).map(move |i| {
        let angle = (2.0 * PI * i as f32) / n as f32 - PI / 2.0;
        Vec2::new(angle.cos(), angle.sin()) * size
    })
}

pub fn star(size: f32, n: usize) -> impl Iterator<Item = Vec2> {
    (0..=2 * n).map(move |i| {
        let angle = (PI * i as f32) / n as f32 - PI / 2.0;
        let radius = if i % 2 == 0 { size } else { size / 2.0 };
        Vec2::new(angle.cos(), angle.sin()) * radius
    })
}

pub fn lissajous(size: f32, steps: usize, n: usize, m: usize) -> impl Iterator<Item = Vec2> {
    (0..=steps).map(move |i| {
        let t = PI / 2.0 - 2.0 * PI * i as f32 / steps as f32;
        let x = (t * n as f32).cos();
        let y = (t * m as f32).sin();
        Vec2::new(x, y) * size
    })
}
