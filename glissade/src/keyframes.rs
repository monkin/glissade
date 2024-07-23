use crate::{Animation, Easing, Mix, Time, TimeDiff};
use std::fmt::Debug;
use std::marker::PhantomData;

/// A transition of a value over time. It works like an animation template, or set of keyframes.
/// A good point to start building `Animation` is the [`keyframes`] function.
pub trait Keyframes<T: Clone, X: Time> {
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
        T: Mix + Clone + PartialEq,
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
        T: Mix + Clone + PartialEq,
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
        T: Mix + Clone + PartialEq,
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
// NoneKeyframes

/// An animation that stays at a single value.
#[derive(Clone)]
pub struct NoneKeyframes<T: Clone, X: Time> {
    value: T,
    duration: X::Duration,
}

impl<T: Clone + Debug, X: Time> Debug for NoneKeyframes<T, X>
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

impl<T: Clone, X: Time> NoneKeyframes<T, X> {
    pub fn new(value: T, duration: X::Duration) -> Self {
        Self { value, duration }
    }
}

impl<T: Clone, X: Time> Keyframes<T, X> for NoneKeyframes<T, X> {
    fn get(&self, _offset: X::Duration) -> T {
        self.value.clone()
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }
}

impl<T: Clone + Copy, X: Time> Copy for NoneKeyframes<T, X> {}

//----------------------------------------------------------------
// LinearKeyframes

/// An animation that linearly interpolates between two values.
#[derive(Clone)]
pub struct LinearKeyframes<T: Mix + Clone + PartialEq, X: Time> {
    v1: T,
    v2: T,
    duration: X::Duration,
}

impl<T: Mix + Clone + PartialEq + Debug, X: Time> Debug for LinearKeyframes<T, X>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinearKeyframes")
            .field("v1", &self.v1)
            .field("v2", &self.v2)
            .field("duration", &self.duration)
            .finish()
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> LinearKeyframes<T, X> {
    pub fn new(v1: T, v2: T, duration: X::Duration) -> Self {
        Self { v1, v2, duration }
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> Keyframes<T, X> for LinearKeyframes<T, X> {
    fn get(&self, offset: X::Duration) -> T {
        let t = offset.as_f32() / self.duration.as_f32();
        self.v1.clone().mix(self.v2.clone(), t)
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }
}

impl<T: Mix + Clone + PartialEq + Copy, X: Time> Copy for LinearKeyframes<T, X> {}

//----------------------------------------------------------------
// SequentialKeyframes

/// A sequence of two keyframes set.
#[derive(Clone)]
pub struct SequentialKeyframes<T: Clone, X: Time, S1: Keyframes<T, X>, S2: Keyframes<T, X>> {
    t1: S1,
    t2: S2,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone, X: Time, S1: Keyframes<T, X> + Debug, S2: Keyframes<T, X> + Debug> Debug
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

impl<T: Clone, X: Time, S1: Keyframes<T, X>, S2: Keyframes<T, X>> Keyframes<T, X>
    for SequentialKeyframes<T, X, S1, S2>
{
    fn get(&self, offset: X::Duration) -> T {
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
}

impl<T: Clone, X: Time, S1: Keyframes<T, X>, S2: Keyframes<T, X>>
    SequentialKeyframes<T, X, S1, S2>
{
    pub fn new(t1: S1, t2: S2) -> Self {
        Self {
            t1,
            t2,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Copy, X: Time, S1: Keyframes<T, X> + Copy, S2: Keyframes<T, X> + Copy> Copy
    for SequentialKeyframes<T, X, S1, S2>
{
}

//----------------------------------------------------------------
// EasingKeyframes

/// An animation that eases between two values.
#[derive(Clone)]
pub struct EasingKeyframes<T: Mix + Clone + PartialEq, X: Time> {
    v1: T,
    v2: T,
    duration: X::Duration,
    easing: Easing,
}

impl<T: Mix + Clone + PartialEq + Debug, X: Time> Debug for EasingKeyframes<T, X>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EasingKeyframes")
            .field("v1", &self.v1)
            .field("v2", &self.v2)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .finish()
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> EasingKeyframes<T, X> {
    pub fn new(v1: T, v2: T, duration: X::Duration, easing: Easing) -> Self {
        Self {
            v1,
            v2,
            duration,
            easing,
        }
    }
}

impl<T: Mix + Clone + PartialEq, X: Time> Keyframes<T, X> for EasingKeyframes<T, X> {
    fn get(&self, offset: X::Duration) -> T {
        let t = self.easing.ease(offset.as_f32() / self.duration.as_f32());
        self.v1.clone().mix(self.v2.clone(), t)
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }
}

//----------------------------------------------------------------
// RepeatKeyframes

/// An animation that repeats keyframes indefinitely.
#[derive(Clone)]
pub struct RepeatKeyframes<T: Clone, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone, X: Time, S: Keyframes<T, X> + Debug> Debug for RepeatKeyframes<T, X, S>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepeatKeyframes")
            .field("keyframes", &self.keyframes)
            .finish()
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X>> RepeatKeyframes<T, X, S> {
    pub fn new(keyframes: S) -> Self {
        Self {
            keyframes,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X>> Keyframes<T, X> for RepeatKeyframes<T, X, S> {
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

impl<T: Clone + Copy, X: Time, S: Keyframes<T, X> + Copy> Copy for RepeatKeyframes<T, X, S> {}

//----------------------------------------------------------------
// RepeatNKeyframes

/// An animation that repeats another keyframes n times.
#[derive(Clone)]
pub struct RepeatNKeyframes<T: Clone, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    n: f32,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone, X: Time, S: Keyframes<T, X> + Debug> Debug for RepeatNKeyframes<T, X, S>
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

impl<T: Clone, X: Time, S: Keyframes<T, X>> RepeatNKeyframes<T, X, S> {
    pub fn new(keyframes: S, n: f32) -> Self {
        Self {
            keyframes,
            n,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X>> Keyframes<T, X> for RepeatNKeyframes<T, X, S> {
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

impl<T: Clone + Copy, X: Time, S: Keyframes<T, X> + Copy> Copy for RepeatNKeyframes<T, X, S> {}

//----------------------------------------------------------------
// ReverseKeyframes

/// An animation that reverses the order of keyframes.
#[derive(Clone)]
pub struct ReverseKeyframes<T: Clone, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone, X: Time, S: Keyframes<T, X> + Debug> Debug for ReverseKeyframes<T, X, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReverseKeyframes")
            .field("keyframes", &self.keyframes)
            .finish()
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X>> ReverseKeyframes<T, X, S> {
    pub fn new(keyframes: S) -> Self {
        Self {
            keyframes,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X>> Keyframes<T, X> for ReverseKeyframes<T, X, S> {
    fn get(&self, offset: X::Duration) -> T {
        self.keyframes.get(self.keyframes.duration().sub(offset))
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration()
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X> + Copy> Copy for ReverseKeyframes<T, X, S> {}

//----------------------------------------------------------------
// ScaleKeyframes

/// An animation that scales the time of keyframes.
#[derive(Clone)]
pub struct ScaleKeyframes<T: Clone, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    scale: f32,
    phantom: PhantomData<(T, X)>,
}

impl<T: Clone, X: Time, S: Keyframes<T, X> + Debug> Debug for ScaleKeyframes<T, X, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScaleKeyframes")
            .field("keyframes", &self.keyframes)
            .field("scale", &self.scale)
            .finish()
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X>> ScaleKeyframes<T, X, S> {
    pub fn new(keyframes: S, scale: f32) -> Self {
        Self {
            keyframes,
            scale,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone, X: Time, S: Keyframes<T, X>> Keyframes<T, X> for ScaleKeyframes<T, X, S> {
    fn get(&self, offset: X::Duration) -> T {
        self.keyframes.get(offset.scale(self.scale))
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration().scale(self.scale)
    }
}

impl<T: Clone + Copy, X: Time, S: Keyframes<T, X> + Copy> Copy for ScaleKeyframes<T, X, S> {}

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
