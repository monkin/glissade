use crate::{Keyframes, Mix, Time, TimeDiff};
use std::fmt::Debug;
use std::marker::PhantomData;

/// An animation that repeats keyframes indefinitely.
#[derive(Clone, PartialEq)]
pub struct RepeatKeyframes<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X> + Debug> Debug
    for RepeatKeyframes<T, X, S>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepeatKeyframes")
            .field("keyframes", &self.keyframes)
            .finish()
    }
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> RepeatKeyframes<T, X, S> {
    pub fn new(keyframes: S) -> Self {
        Self {
            keyframes,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> Keyframes<T, X>
    for RepeatKeyframes<T, X, S>
{
    fn get(&self, offset: X::Duration) -> T {
        let scale = offset.as_f32() / self.keyframes.duration().as_f32();
        self.keyframes.get(self.keyframes.duration().scale(scale))
    }

    fn duration(&self) -> X::Duration {
        panic!("RepeatKeyframes has infinite duration");
    }

    fn is_finished(&self, _offset: X::Duration) -> bool {
        false
    }

    fn is_infinite(&self) -> bool {
        true
    }

    fn end_value(&self) -> T {
        panic!("RepeatKeyframes has no end value");
    }
}

impl<T: Clone + Copy + Mix + PartialEq, X: Time, S: Keyframes<T, X> + Copy> Copy
    for RepeatKeyframes<T, X, S>
{
}
