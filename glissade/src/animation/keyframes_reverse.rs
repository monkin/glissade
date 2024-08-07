use crate::{Keyframes, Mix, Time, TimeDiff};
use std::fmt::Debug;
use std::marker::PhantomData;

/// An animation that reverses the order of keyframes.
#[derive(Clone, PartialEq)]
pub struct ReverseKeyframes<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X> + Debug> Debug
    for ReverseKeyframes<T, X, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReverseKeyframes")
            .field("keyframes", &self.keyframes)
            .finish()
    }
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> ReverseKeyframes<T, X, S> {
    pub fn new(keyframes: S) -> Self {
        Self {
            keyframes,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> Keyframes<T, X>
    for ReverseKeyframes<T, X, S>
{
    fn get(&self, offset: X::Duration) -> T {
        self.keyframes.get(self.keyframes.duration().sub(offset))
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration()
    }
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X> + Copy> Copy
    for ReverseKeyframes<T, X, S>
{
}
