use crate::Distance;

pub trait Curve<T: Distance> {
    fn value_at(&self, t: f32) -> T;
    fn estimate_length(&self) -> f32;
}
