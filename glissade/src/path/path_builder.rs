use crate::path::bezier::{Bezier, Bezier0, Bezier1, Bezier2, Bezier3};
use crate::path::curve::Curve;
use crate::path::path_struct::Path;
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

    pub fn line_to(mut self, destination: T) -> Self {
        self.segments
            .push(Bezier1::new(self.last_point(), destination).into());
        self
    }

    pub fn quad_to(mut self, control: T, destination: T) -> Self {
        self.segments
            .push(Bezier2::new(self.last_point(), control, destination).into());
        self
    }

    pub fn cubic_to(mut self, control1: T, control2: T, destination: T) -> Self {
        self.segments
            .push(Bezier3::new(self.last_point(), control1, control2, destination).into());
        self
    }

    /// Build a path with the given number of steps and table size.
    ///
    /// Arguments:
    /// * `steps_count` - the number of points along the path used to build the table.
    ///     For curves with many segments and complicated shapes higher values are recommended.
    /// * `table_size` - the size of the table used for fast lookup.
    ///     The higher the value, the more accurate easing of the motion.
    pub fn build(self, steps_count: usize, table_size: usize) -> Path<T> {
        Path::new(self.segments, steps_count, table_size)
    }
}
