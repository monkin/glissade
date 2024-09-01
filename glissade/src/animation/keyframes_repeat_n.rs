use crate::{Keyframes, Time, TimeDiff};
use std::fmt::Debug;
use std::marker::PhantomData;

/// An animation that repeats another keyframes n times.
pub struct RepeatNKeyframes<T, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    n: f32,
    phantom: PhantomData<(T, X)>,
}

impl<T, X: Time, S: Keyframes<T, X> + Debug> Debug for RepeatNKeyframes<T, X, S>
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

impl<T, X: Time, S: Keyframes<T, X> + PartialEq> PartialEq for RepeatNKeyframes<T, X, S> {
    fn eq(&self, other: &Self) -> bool {
        self.keyframes == other.keyframes && self.n == other.n
    }
}

impl<T, X: Time, S: Keyframes<T, X>> RepeatNKeyframes<T, X, S> {
    pub fn new(keyframes: S, n: f32) -> Self {
        Self {
            keyframes,
            n,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, S: Keyframes<T, X>> Keyframes<T, X> for RepeatNKeyframes<T, X, S> {
    fn get(&self, offset: X::Duration) -> T {
        if !self.keyframes.is_finite() {
            return self.keyframes.get(offset);
        }

        let duration = self.keyframes.duration().as_f32();
        let n = offset.as_f32() / duration;

        if n < self.n {
            let step_offset = self.keyframes.duration().scale(n.floor());

            let offset = if step_offset < offset {
                offset.sub(step_offset)
            } else {
                Default::default()
            };
            self.keyframes.get(offset)
        } else {
            self.keyframes.end_value()
        }
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration().scale(self.n)
    }

    fn is_finite(&self) -> bool {
        self.keyframes.is_finite()
    }
}

impl<T, X: Time, S: Keyframes<T, X> + Clone> Clone for RepeatNKeyframes<T, X, S> {
    fn clone(&self) -> Self {
        Self {
            keyframes: self.keyframes.clone(),
            n: self.n,
            phantom: Default::default(),
        }
    }
}

impl<T: Copy, X: Time, S: Keyframes<T, X> + Copy> Copy for RepeatNKeyframes<T, X, S> {}

#[cfg(test)]
mod test {
    use crate::{keyframes, Keyframes};

    #[test]
    fn test_repeat_keyframes() {
        let keyframes = keyframes::from::<f32, f32>(0.0)
            .go_to(10.0, 1.0)
            .repeat_n(2.0);
        assert_eq!(keyframes.get(0.0), 0.0);
        assert_eq!(keyframes.get(0.5), 5.0);
        assert_eq!(keyframes.get(0.75), 7.5);
        assert_eq!(keyframes.get(1.5), 5.0);
        assert_eq!(keyframes.get(2.0), 10.0);
        assert_eq!(keyframes.get(2.1), 10.0);
        assert_eq!(keyframes.get(100.0), 10.0);
    }
}
