use crate::{Keyframes, Mix, Time, TimeDiff};
use std::fmt::Debug;

/// An animation that linearly interpolates between two values.
#[derive(Clone, PartialEq)]
pub struct LinearKeyframes<T: Mix + Clone + PartialEq, X: Time> {
    v1: T,
    v2: T,
    duration: X::Duration,
}

impl<T: Mix + Clone + PartialEq + Debug, X: Time> Debug for LinearKeyframes<T, X>
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

impl<T: Mix + Clone + PartialEq, X: Time> LinearKeyframes<T, X> {
    pub fn new(v1: T, v2: T, duration: X::Duration) -> Self {
        Self { v1, v2, duration }
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> Keyframes<T, X> for LinearKeyframes<T, X> {
    fn get(&self, offset: X::Duration) -> T {
        let t = offset.as_f32() / self.duration.as_f32();
        self.v1.clone().mix(self.v2.clone(), t)
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }
}

impl<T: Mix + Clone + PartialEq + Copy, X: Time> Copy for LinearKeyframes<T, X> {}
