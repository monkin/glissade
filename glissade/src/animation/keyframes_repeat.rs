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
        let n = offset.as_f32() / self.keyframes.duration().as_f32();
        let step_offset = self.keyframes.duration().scale(n.floor());

        let offset = if step_offset < offset {
            offset.sub(step_offset)
        } else {
            Default::default()
        };
        self.keyframes.get(offset)
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

#[cfg(test)]
mod tests {
    use crate::{keyframes, Keyframes};

    #[test]
    fn test_repeat_keyframes() {
        let keyframes = keyframes::<f64, f64>(0.0).go_to(8.0, 1.0).repeat();
        assert_eq!(keyframes.get(0.0), 0.0);
        assert_eq!(keyframes.get(0.5), 4.0);
        assert_eq!(keyframes.get(0.75), 6.0);
        assert_eq!(keyframes.get(1.5), 4.0);
        assert_eq!(keyframes.get(2.25), 2.0);
        assert_eq!(keyframes.get(2.5), 4.0);
        assert_eq!(keyframes.get(8.25), 2.0);
    }
}
