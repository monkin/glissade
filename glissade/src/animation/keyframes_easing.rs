use crate::{Easing, Keyframes, Mix, Time, TimeDiff};
use std::fmt::Debug;

/// An animation that eases between two values.
#[derive(Clone, PartialEq)]
pub struct EasingKeyframes<T: Mix + Clone + PartialEq, X: Time> {
    v1: T,
    v2: T,
    duration: X::Duration,
    easing: Easing,
}

impl<T: Mix + Clone + PartialEq + Debug, X: Time> Debug for EasingKeyframes<T, X>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EasingKeyframes")
            .field("v1", &self.v1)
            .field("v2", &self.v2)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .finish()
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> EasingKeyframes<T, X> {
    pub fn new(v1: T, v2: T, duration: X::Duration, easing: Easing) -> Self {
        Self {
            v1,
            v2,
            duration,
            easing,
        }
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> Keyframes<T, X> for EasingKeyframes<T, X> {
    fn get(&self, offset: X::Duration) -> T {
        if offset >= self.duration {
            return self.v2.clone();
        }
        let t = self.easing.ease(offset.as_f32() / self.duration.as_f32());
        self.v1.clone().mix(self.v2.clone(), t)
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }
}
