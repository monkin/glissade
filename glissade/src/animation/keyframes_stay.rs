use crate::{Keyframes, Time};
use std::fmt::Debug;

/// An animation that stays at a single value.
#[derive(Clone)]
pub struct StayKeyframes<T: Clone, X: Time> {
    value: T,
    duration: X::Duration,
}

impl<T: Clone + Debug, X: Time> Debug for StayKeyframes<T, X>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoneKeyframes")
            .field("value", &self.value)
            .field("duration", &self.duration)
            .finish()
    }
}

impl<T: Clone + PartialEq, X: Time> PartialEq for StayKeyframes<T, X> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.duration == other.duration
    }
}

impl<T: Clone, X: Time> StayKeyframes<T, X> {
    pub fn new(value: T, duration: X::Duration) -> Self {
        Self { value, duration }
    }
}

impl<T: Clone, X: Time> Keyframes<T, X> for StayKeyframes<T, X> {
    fn get(&self, _offset: X::Duration) -> T {
        self.value.clone()
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }

    fn is_finite(&self) -> bool {
        true
    }
}

impl<T: Clone + Copy, X: Time> Copy for StayKeyframes<T, X> {}
