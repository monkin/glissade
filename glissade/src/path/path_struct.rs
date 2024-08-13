use crate::path::bezier::Bezier;
use crate::smooth_array::SmoothArray;
use crate::{Distance, Mix};

pub struct Path<T: Distance + Mix + Clone> {
    segments: Vec<Bezier<T>>,
    indexes: SmoothArray,
}

impl<T: Mix + Distance + Clone> Path<T> {
    pub fn new(segments: Vec<Bezier<T>>, steps_count: usize, table_size: usize) -> Self {
        let mut indexes = SmoothArray::new(table_size);
        let mut total_length = 0.0;

        Path { segments, indexes }
    }
}
