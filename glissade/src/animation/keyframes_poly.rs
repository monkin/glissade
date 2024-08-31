use crate::poly::Poly;
use crate::{Distance, Easing, Keyframes, Mix, Time, TimeDiff};
use std::fmt::Debug;

#[derive(Clone)]
pub struct PolyKeyframes<T: Clone + Mix + Distance, X: Time> {
    poly: Poly<T>,
    duration: X::Duration,
    easing: Easing,
}

impl<T, X> Debug for PolyKeyframes<T, X>
where
    T: Clone + Mix + Distance + Debug,
    X: Time,
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PolyKeyframes")
            .field("poly", &self.poly)
            .field("duration", &self.duration)
            .finish()
    }
}

impl<T, X> PartialEq for PolyKeyframes<T, X>
where
    T: Clone + Mix + Distance + PartialEq,
    X: Time,
{
    fn eq(&self, other: &Self) -> bool {
        self.poly == other.poly && self.duration == other.duration
    }
}

impl<T: Clone + Mix + Distance, X: Time> PolyKeyframes<T, X> {
    pub fn new(points: Vec<T>, duration: X::Duration, easing: Easing) -> Self {
        Self {
            poly: Poly::new(points),
            duration,
            easing,
        }
    }
}

impl<T: Clone + Mix + Distance, X: Time> Keyframes<T, X> for PolyKeyframes<T, X> {
    fn get(&self, offset: X::Duration) -> T {
        self.poly
            .value_at(self.easing.ease(offset.as_f32() / self.duration.as_f32()))
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }

    fn is_finite(&self) -> bool {
        true
    }
}
