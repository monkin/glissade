use crate::{Keyframes, Time, TimeDiff};
use std::fmt::Debug;
use std::marker::PhantomData;

/// A sequence of two keyframes set.
pub struct SequentialKeyframes<T, X: Time, S1: Keyframes<T, X>, S2: Keyframes<T, X>> {
    t1: S1,
    t2: S2,
    phantom: PhantomData<(T, X)>,
}

impl<T, X: Time, S1: Keyframes<T, X> + Debug, S2: Keyframes<T, X> + Debug> Debug
    for SequentialKeyframes<T, X, S1, S2>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequentialKeyframes")
            .field("t1", &self.t1)
            .field("t2", &self.t2)
            .finish()
    }
}

impl<T, X: Time, S1: Keyframes<T, X> + PartialEq, S2: Keyframes<T, X> + PartialEq> PartialEq
    for SequentialKeyframes<T, X, S1, S2>
{
    fn eq(&self, other: &Self) -> bool {
        self.t1 == other.t1 && self.t2 == other.t2
    }
}

impl<T, X: Time, S1: Keyframes<T, X>, S2: Keyframes<T, X>> Keyframes<T, X>
    for SequentialKeyframes<T, X, S1, S2>
{
    fn get(&self, offset: X::Duration) -> T {
        if !self.t1.is_finite() {
            return self.t1.get(offset);
        }

        let t1 = self.t1.duration();
        if offset < t1 {
            self.t1.get(offset)
        } else {
            self.t2.get(offset.sub(t1))
        }
    }

    fn duration(&self) -> X::Duration {
        self.t1.duration().add(self.t2.duration())
    }

    fn is_finite(&self) -> bool {
        self.t1.is_finite() && self.t2.is_finite()
    }
}

impl<T, X: Time, S1: Keyframes<T, X>, S2: Keyframes<T, X>> SequentialKeyframes<T, X, S1, S2> {
    pub fn new(t1: S1, t2: S2) -> Self {
        Self {
            t1,
            t2,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, S1: Keyframes<T, X> + Clone, S2: Keyframes<T, X> + Clone> Clone
    for SequentialKeyframes<T, X, S1, S2>
{
    fn clone(&self) -> Self {
        Self {
            t1: self.t1.clone(),
            t2: self.t2.clone(),
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, S1: Keyframes<T, X> + Copy, S2: Keyframes<T, X> + Copy> Copy
    for SequentialKeyframes<T, X, S1, S2>
{
}
