use crate::{Keyframes, Mix, Time, TimeDiff};
use std::fmt::Debug;
use std::marker::PhantomData;

/// An animation that repeats another keyframes n times.
#[derive(Clone, PartialEq)]
pub struct RepeatNKeyframes<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    n: f32,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X> + Debug> Debug
    for RepeatNKeyframes<T, X, S>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepeatNKeyframes")
            .field("keyframes", &self.keyframes)
            .field("n", &self.n)
            .finish()
    }
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> RepeatNKeyframes<T, X, S> {
    pub fn new(keyframes: S, n: f32) -> Self {
        Self {
            keyframes,
            n,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Mix + PartialEq, X: Time, S: Keyframes<T, X>> Keyframes<T, X>
    for RepeatNKeyframes<T, X, S>
{
    fn get(&self, offset: X::Duration) -> T {
        let duration = self.keyframes.duration().as_f32();
        let n = offset.as_f32() / duration;
        if n < self.n {
            self.keyframes
                .get(offset.sub(self.keyframes.duration().scale(n.floor())))
        } else {
            self.keyframes.end_value()
        }
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration().scale(self.n)
    }
}

impl<T: Clone + Copy + Mix + PartialEq, X: Time, S: Keyframes<T, X> + Copy> Copy
    for RepeatNKeyframes<T, X, S>
{
}
