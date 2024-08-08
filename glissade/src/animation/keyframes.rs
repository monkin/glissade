use super::animation_struct::Animation;
use super::keyframes_easing::EasingKeyframes;
use super::keyframes_linear::LinearKeyframes;
use super::keyframes_none::NoneKeyframes;
use super::keyframes_repeat::RepeatKeyframes;
use super::keyframes_repeat_n::RepeatNKeyframes;
use super::keyframes_reverse::ReverseKeyframes;
use super::keyframes_scale::ScaleKeyframes;
use super::keyframes_sequential::SequentialKeyframes;
use crate::{Easing, Mix, Time, TimeDiff};

/// A transition of a value over time. It works like an animation template, or set of keyframes.
/// A good point to start building `Animation` is the [`keyframes`] function.
pub trait Keyframes<T: Clone + Mix + PartialEq, X: Time> {
    /// Get the value at a specific time offset from the start.
    /// If the offset is greater than the duration, the value at the end of the animation is returned.
    fn get(&self, offset: X::Duration) -> T;

    /// Get the duration of the animation.
    /// If the animation is infinite, it will panic.
    fn duration(&self) -> X::Duration;

    /// Check if the animation is finished at the given offset.
    fn is_finished(&self, offset: X::Duration) -> bool {
        offset >= self.duration()
    }

    /// Check if the animation is infinite.
    fn is_infinite(&self) -> bool {
        false
    }

    /// Check if the animation is finite.
    fn is_finite(&self) -> bool {
        !self.is_infinite()
    }

    /// Get the value of the animation at the start.
    fn start_value(&self) -> T {
        self.get(Default::default())
    }

    /// Get the value of the animation at the end.
    /// If the animation is infinite, it will panic.
    fn end_value(&self) -> T {
        self.get(self.duration())
    }

    /// Create an animation that stays at the end value for the given duration.
    fn stay(self, duration: X::Duration) -> SequentialKeyframes<T, X, Self, NoneKeyframes<T, X>>
    where
        Self: Sized,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(self, NoneKeyframes::new(end_value, duration))
    }

    /// Create an animation that linearly interpolates between the end value and the target value.
    fn go_to(
        self,
        target: T,
        duration: X::Duration,
    ) -> SequentialKeyframes<T, X, Self, LinearKeyframes<T, X>>
    where
        Self: Sized,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(self, LinearKeyframes::new(end_value, target, duration))
    }

    /// Create an animation that eases between the end value and the target value.
    fn ease_to(
        self,
        target: T,
        duration: X::Duration,
        easing: Easing,
    ) -> SequentialKeyframes<T, X, Self, EasingKeyframes<T, X>>
    where
        Self: Sized,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(
            self,
            EasingKeyframes::new(end_value, target, duration, easing),
        )
    }

    /// Create an animation that repeats the given keyframes indefinitely.
    fn repeat(self) -> RepeatKeyframes<T, X, Self>
    where
        Self: Sized,
    {
        RepeatKeyframes::new(self)
    }

    /// Create an animation that repeats the given keyframes n times.
    /// * `n` - The number of times to repeat the keyframes. It can be not integer, and repeat the keyframes partially.
    fn repeat_n(self, n: f32) -> RepeatNKeyframes<T, X, Self>
    where
        Self: Sized,
    {
        RepeatNKeyframes::new(self, n)
    }

    /// Inverse keyframes order.
    fn reverse(self) -> ReverseKeyframes<T, X, Self>
    where
        Self: Sized,
    {
        ReverseKeyframes::new(self)
    }

    /// Scale the time of the animation by the given factor.
    fn scale(self, scale: f32) -> ScaleKeyframes<T, X, Self>
    where
        Self: Sized,
    {
        ScaleKeyframes::new(self, scale)
    }

    /// Scale the time of the animation to the given duration.
    fn scale_to(self, new_duration: X::Duration) -> ScaleKeyframes<T, X, Self>
    where
        Self: Sized,
    {
        let scale = if self.duration() == Default::default() {
            1.0
        } else {
            new_duration.as_f32() / self.duration().as_f32()
        };

        ScaleKeyframes::new(self, scale)
    }

    /// Concatenate two keyframes set.
    fn then<S: Keyframes<T, X>>(self, other: S) -> SequentialKeyframes<T, X, Self, S>
    where
        Self: Sized,
    {
        SequentialKeyframes::new(self, other)
    }

    /// Run keyframes at a specific time.
    /// * `start_time` - The time to start the transition, usually `Instant::now()`.
    fn run(self, start_time: X) -> Animation<T, X, Self>
    where
        Self: Sized,
    {
        Animation::start(self, start_time)
    }
}

/// Start `Animation` constructing with this function. It receives the initial value.
/// * `value` - The value to start at.
///
/// See [`Keyframes`] trait methods for more ways of adding next frames and building an animation.
///
/// # Examples
///
/// ```
/// use std::time::Instant;
/// use glissade::{keyframes, Keyframes};
/// use web_time::Duration;
///
/// let transition = keyframes::<f64, Instant>(5.0)
///     .stay(Duration::from_secs(1))
///     .go_to(9.0, Duration::from_secs(4))
///     .repeat_n(2.0);
///
/// assert_eq!(transition.get(Duration::from_secs(0)), 5.0);
/// assert_eq!(transition.get(Duration::from_secs(1)), 5.0);
/// assert_eq!(transition.get(Duration::from_secs(2)), 6.0);
/// assert_eq!(transition.get(Duration::from_secs(3)), 7.0);
/// assert_eq!(transition.get(Duration::from_secs(4)), 8.0);
/// assert_eq!(transition.get(Duration::from_millis(4500)), 8.5);
/// assert_eq!(transition.get(Duration::from_secs(6)), 5.0);
/// assert_eq!(transition.get(Duration::from_secs(74)), 9.0);
/// ```
pub fn keyframes<T: Mix + Clone + PartialEq, X: Time>(start_value: T) -> NoneKeyframes<T, X> {
    NoneKeyframes::new(start_value, Default::default())
}

//----------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::easing::Easing;
    use crate::mix::Mix;
    use std::time::{Duration, Instant};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestItem(f32);

    impl Mix for TestItem {
        fn mix(self, other: Self, t: f32) -> Self {
            TestItem(self.0.mix(other.0, t))
        }
    }

    const ZERO_DURATION: Duration = Duration::from_secs(0);
    const ONE_SECOND: Duration = Duration::from_secs(1);
    const HALF_SECOND: Duration = Duration::from_millis(500);
    const ONE_AND_HALF_SECONDS: Duration = Duration::from_millis(1500);
    const TWO_SECONDS: Duration = Duration::from_secs(2);

    #[test]
    fn none_keyframes() {
        let keyframes: NoneKeyframes<TestItem, Instant> =
            NoneKeyframes::new(TestItem(0.0), Duration::from_secs(1));
        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.0));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(0.0));
    }

    #[test]
    fn linear_keyframes() {
        let keyframes =
            LinearKeyframes::<TestItem, Instant>::new(TestItem(0.0), TestItem(1.0), ONE_SECOND);
        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(1.0));
    }

    #[test]
    fn sequential_keyframes() {
        let keyframes = SequentialKeyframes::new(
            LinearKeyframes::<TestItem, Instant>::new(TestItem(0.0), TestItem(1.0), ONE_SECOND),
            LinearKeyframes::new(TestItem(1.0), TestItem(0.0), ONE_SECOND),
        );
        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(1.0));
        assert_eq!(keyframes.get(ONE_AND_HALF_SECONDS), TestItem(0.5));
        assert_eq!(keyframes.get(TWO_SECONDS), TestItem(0.0));
    }

    #[test]
    fn easing_keyframes() {
        let keyframes = EasingKeyframes::<TestItem, Instant>::new(
            TestItem(0.0),
            TestItem(1.0),
            ONE_SECOND,
            Easing::QuadraticIn,
        );
        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.25));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(1.0));
    }

    #[test]
    fn reversed_keyframes() {
        let keyframes = keyframes::<TestItem, Instant>(TestItem(0.0))
            .go_to(TestItem(1.0), ONE_SECOND)
            .reverse();

        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(1.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(0.0));
    }
}
