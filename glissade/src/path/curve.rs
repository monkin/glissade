use crate::Distance;

pub trait Curve<T: Distance> {
    fn value_at(&self, t: f32) -> T;
    fn estimate_length(&self) -> f32;

    fn first_point(&self) -> T {
        self.value_at(0.0)
    }

    fn last_point(&self) -> T {
        self.value_at(1.0)
    }
}
