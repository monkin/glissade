use crate::{Keyframes, Mix, Time};
use std::fmt::Debug;

/// An animation that linearly interpolates between two values.
#[derive(Clone)]
pub struct LinearKeyframes<T: Mix + Clone, X: Time> {
    v1: T,
    v2: T,
    duration: X::Duration,
}

impl<T: Mix + Clone + Debug, X: Time> Debug for LinearKeyframes<T, X>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinearKeyframes")
            .field("v1", &self.v1)
            .field("v2", &self.v2)
            .field("duration", &self.duration)
            .finish()
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> PartialEq for LinearKeyframes<T, X> {
    fn eq(&self, other: &Self) -> bool {
        self.v1 == other.v1 && self.v2 == other.v2 && self.duration == other.duration
    }
}

impl<T: Mix + Clone, X: Time> LinearKeyframes<T, X> {
    pub fn new(v1: T, v2: T, duration: X::Duration) -> Self {
        Self { v1, v2, duration }
    }
}

impl<T: Mix + Clone, X: Time> Keyframes<T, X> for LinearKeyframes<T, X> {
    fn get(&self, offset: X::Duration) -> T {
        if offset < Default::default() {
            self.v1.clone()
        } else if offset >= self.duration {
            self.v2.clone()
        } else {
            let t = X::duration_as_f32(offset) / X::duration_as_f32(self.duration);
            self.v1.clone().mix(self.v2.clone(), t)
        }
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }

    fn is_finite(&self) -> bool {
        true
    }
}

impl<T: Mix + Clone + Copy, X: Time> Copy for LinearKeyframes<T, X> {}
