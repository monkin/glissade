use crate::path::bezier::{Bezier, Bezier0, Bezier1, Bezier2, Bezier3};
use crate::path::curve::Curve;
use crate::{Distance, Mix};

pub struct PathBuilder<T: Mix + Distance + Clone> {
    segments: Vec<Bezier<T>>,
}

impl<T: Mix + Distance + Clone> PathBuilder<T> {
    fn last_point(&self) -> T {
        self.segments.last().unwrap().last_point()
    }

    pub fn new(start_point: T) -> Self {
        Self {
            segments: vec![Bezier0::new(start_point).into()],
        }
    }

    pub fn with_capacity(start_point: T, capacity: usize) -> Self {
        let mut segments = Vec::with_capacity(capacity);
        segments.push(Bezier0::new(start_point).into());
        Self { segments }
    }

    pub fn line_to(&mut self, destination: T) {
        self.segments
            .push(Bezier1::new(self.last_point(), destination).into());
    }

    pub fn quad_to(&mut self, control: T, destination: T) {
        self.segments
            .push(Bezier2::new(self.last_point(), control, destination).into());
    }

    pub fn cubic_to(&mut self, control1: T, control2: T, destination: T) {
        self.segments
            .push(Bezier3::new(self.last_point(), control1, control2, destination).into());
    }
}
