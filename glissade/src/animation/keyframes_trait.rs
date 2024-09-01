use super::animation_struct::Animation;
use super::keyframes_easing::EasingKeyframes;
use super::keyframes_linear::LinearKeyframes;
use super::keyframes_repeat::RepeatKeyframes;
use super::keyframes_repeat_n::RepeatNKeyframes;
use super::keyframes_reverse::ReverseKeyframes;
use super::keyframes_scale::ScaleKeyframes;
use super::keyframes_sequential::SequentialKeyframes;
use super::keyframes_stay::StayKeyframes;
use crate::animation::keyframes_function::FunctionKeyframes;
use crate::animation::keyframes_poly::PolyKeyframes;
use crate::animation::keyframes_slice::SliceKeyframes;
use crate::{Distance, Easing, Mix, Time, TimeDiff};
use std::iter::once;

/// A transition of a value over time. It works like an animation template, or set of keyframes.
pub trait Keyframes<T, X: Time> {
    /// Get the value at a specific time offset from the start.
    /// If the offset is greater than the duration, the value at the end of the animation is returned.
    fn get(&self, offset: X::Duration) -> T;

    /// Get the duration of the animation.
    /// If the animation is infinite, it will panic.
    fn duration(&self) -> X::Duration;

    /// Check if the animation is finished at the given offset.
    fn is_finished(&self, offset: X::Duration) -> bool {
        self.is_finite() && self.duration() <= offset
    }

    /// Check if the animation is finite.
    fn is_finite(&self) -> bool;

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
    fn stay(self, duration: X::Duration) -> SequentialKeyframes<T, X, Self, StayKeyframes<T, X>>
    where
        T: Clone,
        Self: Sized,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(self, StayKeyframes::new(end_value, duration))
    }

    /// Create an animation that linearly interpolates between the end value and the target value.
    fn go_to(
        self,
        target: T,
        duration: X::Duration,
    ) -> SequentialKeyframes<T, X, Self, LinearKeyframes<T, X>>
    where
        T: Mix + Clone,
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
        T: Mix + Clone,
        Self: Sized,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(
            self,
            EasingKeyframes::new(end_value, target, duration, easing),
        )
    }

    /// Create an animation that follows the given polynomial curve with easing.
    fn poly_to(
        self,
        points: impl IntoIterator<Item = T>,
        duration: X::Duration,
        easing: Easing,
    ) -> SequentialKeyframes<T, X, Self, PolyKeyframes<T, X>>
    where
        Self: Sized,
        T: Mix + Clone + Distance,
    {
        let points = once(self.end_value()).chain(points).collect();
        SequentialKeyframes::new(self, PolyKeyframes::new(points, duration, easing))
    }

    /// Follows the given function.
    fn function<F: Fn(X::Duration) -> T>(
        self,
        function: F,
        duration: X::Duration,
    ) -> SequentialKeyframes<T, X, Self, FunctionKeyframes<T, X, F>>
    where
        Self: Sized,
    {
        SequentialKeyframes::new(self, FunctionKeyframes::new(function, duration))
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

    /// Get a slice of the keyframes from the start to the end.
    fn slice(self, start_offset: X::Duration, end_offset: X::Duration) -> SliceKeyframes<T, X, Self>
    where
        Self: Sized,
    {
        SliceKeyframes::new(self, (start_offset, end_offset))
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

/// Start `Animation` constructing with this module.
/// * `keyframes::from` - to start keyframes at a specific point.
/// * `keyframes::stay` - to create a keyframes that stays at point for a while.
/// * `keyframes::line` - to create a keyframes that linearly goes from one point to another.
/// * `keyframes::ease` - to create a keyframes that goes from one point to another with easing.
/// * `keyframes::poly` - to create a keyframes that goes along a path.
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
/// let transition = keyframes::stay::<f64, Instant>(5.0, Duration::from_secs(1))
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
pub mod keyframes {
    use super::Keyframes;
    use crate::animation::keyframes_easing::EasingKeyframes;
    use crate::animation::keyframes_function::FunctionKeyframes;
    use crate::animation::keyframes_linear::LinearKeyframes;
    use crate::animation::keyframes_poly::PolyKeyframes;
    use crate::animation::keyframes_stay::StayKeyframes;
    use crate::{Distance, Easing, Mix, Time};

    pub fn from<T: Clone, X: Time>(point: T) -> impl Keyframes<T, X> {
        stay(point, Default::default())
    }

    /// Create a new keyframes that stays at a single value.
    pub fn stay<T: Clone, X: Time>(value: T, duration: X::Duration) -> impl Keyframes<T, X> {
        StayKeyframes::new(value, duration)
    }

    /// Create a new keyframes that linearly go from one value to another.
    pub fn line<T: Mix + Clone, X: Time>(
        start: T,
        end: T,
        duration: X::Duration,
    ) -> impl Keyframes<T, X> {
        LinearKeyframes::new(start, end, duration)
    }

    /// Create a new keyframes that go from one value to another with easing.
    pub fn ease<T: Mix + Clone, X: Time>(
        start: T,
        end: T,
        duration: X::Duration,
        easing: Easing,
    ) -> impl Keyframes<T, X> {
        EasingKeyframes::new(start, end, duration, easing)
    }

    /// Create a new keyframes that goes along a path.
    pub fn poly<T: Mix + Distance + Clone, X: Time>(
        points: Vec<T>,
        duration: X::Duration,
        easing: Easing,
    ) -> impl Keyframes<T, X> {
        PolyKeyframes::new(points, duration, easing)
    }

    /// Create a new keyframes that goes along functionally defined path.
    pub fn function<T, X, F>(f: F, duration: X::Duration) -> impl Keyframes<T, X>
    where
        X: Time,
        F: Fn(X::Duration) -> T,
    {
        FunctionKeyframes::new(f, duration)
    }
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
        let keyframes: StayKeyframes<TestItem, Instant> =
            StayKeyframes::new(TestItem(0.0), Duration::from_secs(1));
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
        let keyframes = keyframes::from::<TestItem, Instant>(TestItem(0.0))
            .go_to(TestItem(1.0), ONE_SECOND)
            .reverse();

        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(1.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(0.0));
    }
}
