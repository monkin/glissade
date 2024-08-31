use crate::{Easing, Keyframes, Mix, Time, TimeDiff};
use std::fmt::Debug;

/// An animation that eases between two values.
#[derive(Clone)]
pub struct EasingKeyframes<T: Mix + Clone, X: Time> {
    v1: T,
    v2: T,
    duration: X::Duration,
    easing: Easing,
}

impl<T: Mix + Clone + Debug, X: Time> Debug for EasingKeyframes<T, X>
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

impl<T: Mix + Clone + PartialEq, X: Time> PartialEq for EasingKeyframes<T, X> {
    fn eq(&self, other: &Self) -> bool {
        self.v1 == other.v1
            && self.v2 == other.v2
            && self.duration == other.duration
            && self.easing == other.easing
    }
}

impl<T: Mix + Clone, X: Time> EasingKeyframes<T, X> {
    pub fn new(v1: T, v2: T, duration: X::Duration, easing: Easing) -> Self {
        Self {
            v1,
            v2,
            duration,
            easing,
        }
    }
}

impl<T: Mix + Clone, X: Time> Keyframes<T, X> for EasingKeyframes<T, X> {
    fn get(&self, offset: X::Duration) -> T {
        if offset < Default::default() {
            self.v1.clone()
        } else if offset >= self.duration {
            self.v2.clone()
        } else {
            let t = self.easing.ease(offset.as_f32() / self.duration.as_f32());
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
