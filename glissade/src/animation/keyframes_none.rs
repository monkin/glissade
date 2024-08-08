use crate::{Keyframes, Mix, Time};
use std::fmt::Debug;

/// An animation that stays at a single value.
#[derive(Clone, PartialEq)]
pub struct NoneKeyframes<T: Clone + Mix + PartialEq, X: Time> {
    value: T,
    duration: X::Duration,
}

impl<T: Clone + Mix + PartialEq + Debug, X: Time> Debug for NoneKeyframes<T, X>
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

impl<T: Clone + Mix + PartialEq, X: Time> NoneKeyframes<T, X> {
    pub fn new(value: T, duration: X::Duration) -> Self {
        Self { value, duration }
    }
}

impl<T: Clone + Mix + PartialEq, X: Time> Keyframes<T, X> for NoneKeyframes<T, X> {
    fn get(&self, _offset: X::Duration) -> T {
        self.value.clone()
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }
}

impl<T: Clone + Mix + PartialEq + Copy, X: Time> Copy for NoneKeyframes<T, X> {}
