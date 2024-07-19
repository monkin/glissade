use crate::animated_item::AnimatedItem;
use crate::animation::Animation;
use crate::easing::Easing;
use std::fmt::Debug;
use std::marker::PhantomData;
use web_time::{Duration, Instant};

/// A transition of a value over time. It works like an animation template.
/// A good point to start building transition is the [`keyframes`] function.
pub trait Keyframes<T: Clone + Sized>: Sized {
    /// Get the value of the transition at a specific time offset from the start.
    /// If the offset is greater than the duration, the value at the end of the transition is returned.
    fn get(&self, offset: Duration) -> T;

    /// Get the duration of the transition.
    /// If the transition is infinite, it will panic.
    fn duration(&self) -> Duration;

    /// Check if the transition is finished at the given offset.
    fn is_finished(&self, offset: Duration) -> bool {
        offset >= self.duration()
    }

    /// Check if the transition is infinite.
    fn is_infinite(&self) -> bool {
        false
    }

    /// Get the value of the transition at the start.
    fn start_value(&self) -> T {
        self.get(Duration::default())
    }

    /// Get the value of the transition at the end.
    /// If the transition is infinite, it will panic.
    fn end_value(&self) -> T {
        self.get(self.duration())
    }

    /// Create a transition that stays at the end value for the given duration.
    fn stay(self, duration: Duration) -> SequentialKeyframes<T, Self, NoneKeyframes<T>>
    where
        Self: Sized,
        T: AnimatedItem,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(self, NoneKeyframes::new(end_value, duration))
    }

    /// Create a transition that linearly interpolates between the end value and the target value.
    fn go_to(
        self,
        target: T,
        duration: Duration,
    ) -> SequentialKeyframes<T, Self, LinearKeyframes<T>>
    where
        Self: Sized,
        T: AnimatedItem,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(self, LinearKeyframes::new(end_value, target, duration))
    }

    /// Create a transition that eases between the end value and the target value.
    fn ease_to(
        self,
        target: T,
        duration: Duration,
        easing: Easing,
    ) -> SequentialKeyframes<T, Self, EasingKeyframes<T>>
    where
        Self: Sized,
        T: AnimatedItem,
    {
        let end_value = self.end_value();
        SequentialKeyframes::new(
            self,
            EasingKeyframes::new(end_value, target, duration, easing),
        )
    }

    /// Create a transition that repeats the given transition indefinitely.
    fn repeat(self) -> RepeatKeyframes<T, Self>
    where
        Self: Sized,
    {
        RepeatKeyframes::new(self)
    }

    /// Create a transition that repeats the given transition n times.
    fn repeat_n(self, n: f32) -> RepeatNKeyframes<T, Self>
    where
        Self: Sized,
    {
        RepeatNKeyframes::new(self, n)
    }

    /// Create a transition that reverses the given transition.
    fn reverse(self) -> ReverseKeyframes<T, Self>
    where
        Self: Sized,
    {
        ReverseKeyframes::new(self)
    }

    /// Scale the time of the transition by the given factor.
    fn scale(self, scale: f32) -> ScaleKeyframes<T, Self>
    where
        Self: Sized,
    {
        ScaleKeyframes::new(self, scale)
    }

    /// Scale the time of the transition to the given duration.
    fn scale_to(self, new_duration: Duration) -> ScaleKeyframes<T, Self>
    where
        Self: Sized,
    {
        let scale = if self.duration().is_zero() {
            1.0
        } else {
            new_duration.as_secs_f32() / self.duration().as_secs_f32()
        };

        ScaleKeyframes::new(self, scale)
    }

    /// Similar to Vec::map, creates a transition that applies the given function to the value at each point in time.
    fn map<F: Fn(T) -> T>(self, map: F) -> MapKeyframes<T, Self, F>
    where
        Self: Sized,
    {
        MapKeyframes::new(self, map)
    }

    /// Concatenate two transitions.
    fn then<S: Keyframes<T>>(self, other: S) -> SequentialKeyframes<T, Self, S>
    where
        Self: Sized,
    {
        SequentialKeyframes::new(self, other)
    }

    /// Run keyframes at a specific time.
    /// * `start_time` - The time to start the transition, usually `Instant::now()`.
    fn run(self, start_time: Instant) -> Animation<T, Self> {
        Animation::start(self, start_time)
    }
}

/// Start transition constructing with this function. It receives the initial value.
/// * `value` - The value to start at.
///
/// See [`Keyframes`] trait methods for more options of constructing transitions.
///
/// # Examples
///
/// ```
/// use glissade::{keyframes, Keyframes};
/// use web_time::Duration;
///
/// let transition = keyframes(5.0)
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
pub fn keyframes<T: AnimatedItem>(start_value: T) -> NoneKeyframes<T> {
    NoneKeyframes::new(start_value, Duration::default())
}

//----------------------------------------------------------------
// NoneKeyframes

/// A transition that stays at a single value.
#[derive(Clone)]
pub struct NoneKeyframes<T: Clone + Sized> {
    value: T,
    duration: Duration,
}

impl<T: Clone + Sized + Debug> Debug for NoneKeyframes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoneKeyframes")
            .field("value", &self.value)
            .field("duration", &self.duration)
            .finish()
    }
}

impl<T: Clone + Sized> NoneKeyframes<T> {
    pub fn new(value: T, duration: Duration) -> Self {
        Self { value, duration }
    }
}

impl<T: Clone + Sized> Keyframes<T> for NoneKeyframes<T> {
    fn get(&self, _offset: Duration) -> T {
        self.value.clone()
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl<T: Clone + Sized + Copy> Copy for NoneKeyframes<T> {}

//----------------------------------------------------------------
// LinearKeyframes

/// A transition that linearly interpolates between two values.
#[derive(Clone)]
pub struct LinearKeyframes<T: AnimatedItem> {
    v1: T,
    v2: T,
    duration: Duration,
}

impl<T: AnimatedItem + Debug> Debug for LinearKeyframes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinearKeyframes")
            .field("v1", &self.v1)
            .field("v2", &self.v2)
            .field("duration", &self.duration)
            .finish()
    }
}

impl<T: AnimatedItem> LinearKeyframes<T> {
    pub fn new(v1: T, v2: T, duration: Duration) -> Self {
        Self { v1, v2, duration }
    }
}

impl<T: AnimatedItem> Keyframes<T> for LinearKeyframes<T> {
    fn get(&self, offset: Duration) -> T {
        let t = offset.as_millis() as f32 / self.duration.as_millis() as f32;
        self.v1.clone().mix(self.v2.clone(), t)
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl<T: AnimatedItem + Copy> Copy for LinearKeyframes<T> {}

//----------------------------------------------------------------
// SequentialKeyframes

/// A sequence of two transitions.
#[derive(Clone)]
pub struct SequentialKeyframes<T: Clone + Sized, S1: Keyframes<T>, S2: Keyframes<T>> {
    t1: S1,
    t2: S2,
    phantom: PhantomData<T>,
}

impl<T: Clone + Sized, S1: Keyframes<T> + Debug, S2: Keyframes<T> + Debug> Debug
    for SequentialKeyframes<T, S1, S2>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequentialKeyframes")
            .field("t1", &self.t1)
            .field("t2", &self.t2)
            .finish()
    }
}

impl<T: Clone + Sized, S1: Keyframes<T>, S2: Keyframes<T>> Keyframes<T>
    for SequentialKeyframes<T, S1, S2>
{
    fn get(&self, offset: Duration) -> T {
        let t1 = self.t1.duration();
        if offset < t1 {
            self.t1.get(offset)
        } else {
            self.t2.get(offset - t1)
        }
    }

    fn duration(&self) -> Duration {
        self.t1.duration() + self.t2.duration()
    }
}

impl<T: Clone + Sized, S1: Keyframes<T>, S2: Keyframes<T>> SequentialKeyframes<T, S1, S2> {
    pub fn new(t1: S1, t2: S2) -> Self {
        Self {
            t1,
            t2,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Sized + Copy, S1: Keyframes<T> + Copy, S2: Keyframes<T> + Copy> Copy
    for SequentialKeyframes<T, S1, S2>
{
}

//----------------------------------------------------------------
// EasingKeyframes

/// A transition that eases between two values.
#[derive(Clone)]
pub struct EasingKeyframes<T: AnimatedItem> {
    v1: T,
    v2: T,
    duration: Duration,
    easing: Easing,
}

impl<T: AnimatedItem + Debug> Debug for EasingKeyframes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EasingKeyframes")
            .field("v1", &self.v1)
            .field("v2", &self.v2)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .finish()
    }
}

impl<T: AnimatedItem> EasingKeyframes<T> {
    pub fn new(v1: T, v2: T, duration: Duration, easing: Easing) -> Self {
        Self {
            v1,
            v2,
            duration,
            easing,
        }
    }
}

impl<T: AnimatedItem> Keyframes<T> for EasingKeyframes<T> {
    fn get(&self, offset: Duration) -> T {
        let t = self
            .easing
            .ease(offset.as_millis() as f32 / self.duration.as_millis() as f32);
        self.v1.clone().mix(self.v2.clone(), t)
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

//----------------------------------------------------------------
// RepeatKeyframes

/// A transition that repeats another transition indefinitely.
#[derive(Clone)]
pub struct RepeatKeyframes<T: Clone + Sized, S: Keyframes<T>> {
    keyframes: S,
    phantom: PhantomData<T>,
}

impl<T: Clone + Sized, S: Keyframes<T> + Debug> Debug for RepeatKeyframes<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepeatKeyframes")
            .field("transition", &self.keyframes)
            .finish()
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> RepeatKeyframes<T, S> {
    pub fn new(keyframes: S) -> Self {
        Self {
            keyframes,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> Keyframes<T> for RepeatKeyframes<T, S> {
    fn get(&self, offset: Duration) -> T {
        self.keyframes.get(Duration::from_millis(
            offset.as_millis() as u64 % self.keyframes.duration().as_millis() as u64,
        ))
    }

    fn duration(&self) -> Duration {
        panic!("RepeatTransition has infinite duration");
    }

    fn is_finished(&self, _offset: Duration) -> bool {
        false
    }

    fn is_infinite(&self) -> bool {
        true
    }

    fn end_value(&self) -> T {
        panic!("RepeatTransition has no end value");
    }
}

impl<T: Clone + Sized + Copy, S: Keyframes<T> + Copy> Copy for RepeatKeyframes<T, S> {}

//----------------------------------------------------------------
// RepeatNKeyframes

/// A transition that repeats another transition n times.
#[derive(Clone)]
pub struct RepeatNKeyframes<T: Clone + Sized, S: Keyframes<T>> {
    keyframes: S,
    n: f32,
    phantom: PhantomData<T>,
}

impl<T: Clone + Sized, S: Keyframes<T> + Debug> Debug for RepeatNKeyframes<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepeatNKeyframes")
            .field("transition", &self.keyframes)
            .field("n", &self.n)
            .finish()
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> RepeatNKeyframes<T, S> {
    pub fn new(keyframes: S, n: f32) -> Self {
        Self {
            keyframes,
            n,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> Keyframes<T> for RepeatNKeyframes<T, S> {
    fn get(&self, offset: Duration) -> T {
        let n = (offset.as_millis() as f64 / self.keyframes.duration().as_millis() as f64) as f32;
        if n < self.n {
            self.keyframes.get(Duration::from_millis(
                offset.as_millis() as u64 % self.keyframes.duration().as_millis() as u64,
            ))
        } else {
            self.keyframes.end_value()
        }
    }

    fn duration(&self) -> Duration {
        Duration::from_millis((self.keyframes.duration().as_millis() as f32 * self.n) as u64)
    }
}

impl<T: Clone + Sized + Copy, S: Keyframes<T> + Copy> Copy for RepeatNKeyframes<T, S> {}

//----------------------------------------------------------------
// ReverseKeyframes

/// A transition that reverses the value of another transition.
#[derive(Clone)]
pub struct ReverseKeyframes<T: Clone + Sized, S: Keyframes<T>> {
    keyframes: S,
    phantom: PhantomData<T>,
}

impl<T: Clone + Sized, S: Keyframes<T> + Debug> Debug for ReverseKeyframes<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReverseKeyframes")
            .field("transition", &self.keyframes)
            .finish()
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> ReverseKeyframes<T, S> {
    pub fn new(keyframes: S) -> Self {
        Self {
            keyframes,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> Keyframes<T> for ReverseKeyframes<T, S> {
    fn get(&self, time: Duration) -> T {
        self.keyframes.get(self.keyframes.duration() - time)
    }

    fn duration(&self) -> Duration {
        self.keyframes.duration()
    }
}

impl<T: Clone + Sized + Copy, S: Keyframes<T> + Copy> Copy for ReverseKeyframes<T, S> {}

//----------------------------------------------------------------
// MapKeyframes

/// A transition that maps the value of another transition.
#[derive(Clone)]
pub struct MapKeyframes<T: Clone + Sized, S: Keyframes<T>, F: Fn(T) -> T> {
    keyframes: S,
    map: F,
    phantom: PhantomData<T>,
}

impl<T: Clone + Sized, S: Keyframes<T> + Debug, F: Debug + Fn(T) -> T> Debug
    for MapKeyframes<T, S, F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapKeyframes")
            .field("transition", &self.keyframes)
            .field("map", &self.map)
            .finish()
    }
}

impl<T: Clone + Sized, S: Keyframes<T>, F: Fn(T) -> T> MapKeyframes<T, S, F> {
    pub fn new(keyframes: S, map: F) -> Self {
        Self {
            keyframes,
            map,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Sized, S: Keyframes<T>, F: Fn(T) -> T> Keyframes<T> for MapKeyframes<T, S, F> {
    fn get(&self, time: Duration) -> T {
        (self.map)(self.keyframes.get(time))
    }

    fn duration(&self) -> Duration {
        self.keyframes.duration()
    }
}

impl<T: Clone + Sized + Copy, S: Keyframes<T> + Copy, F: Copy + Fn(T) -> T> Copy
    for MapKeyframes<T, S, F>
{
}

//----------------------------------------------------------------
// ScaleKeyframes

/// A transition that scales the time of another transition.
#[derive(Clone)]
pub struct ScaleKeyframes<T: Clone + Sized, S: Keyframes<T>> {
    keyframes: S,
    scale: f32,
    phantom: PhantomData<T>,
}

impl<T: Clone + Sized, S: Keyframes<T> + Debug> Debug for ScaleKeyframes<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScaleKeyframes")
            .field("transition", &self.keyframes)
            .field("scale", &self.scale)
            .finish()
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> ScaleKeyframes<T, S> {
    pub fn new(keyframes: S, scale: f32) -> Self {
        Self {
            keyframes,
            scale,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Sized, S: Keyframes<T>> Keyframes<T> for ScaleKeyframes<T, S> {
    fn get(&self, time: Duration) -> T {
        self.keyframes.get(Duration::from_millis(
            (time.as_millis() as f32 * self.scale) as u64,
        ))
    }

    fn duration(&self) -> Duration {
        Duration::from_millis((self.keyframes.duration().as_millis() as f32 * self.scale) as u64)
    }
}

impl<T: Clone + Sized + Copy, S: Keyframes<T> + Copy> Copy for ScaleKeyframes<T, S> {}

//----------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::easing::Easing;
    use crate::mix::Mix;

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
    fn none_transition() {
        let keyframes = NoneKeyframes::new(TestItem(0.0), Duration::from_secs(1));
        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.0));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(0.0));
    }

    #[test]
    fn linear_transition() {
        let keyframes = LinearKeyframes::new(TestItem(0.0), TestItem(1.0), ONE_SECOND);
        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(1.0));
    }

    #[test]
    fn sequential_transition() {
        let keyframes = SequentialKeyframes::new(
            LinearKeyframes::new(TestItem(0.0), TestItem(1.0), ONE_SECOND),
            LinearKeyframes::new(TestItem(1.0), TestItem(0.0), ONE_SECOND),
        );
        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(1.0));
        assert_eq!(keyframes.get(ONE_AND_HALF_SECONDS), TestItem(0.5));
        assert_eq!(keyframes.get(TWO_SECONDS), TestItem(0.0));
    }

    #[test]
    fn easing_transition() {
        let keyframes = EasingKeyframes::new(
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
    fn reversed_transition() {
        let keyframes = keyframes(TestItem(0.0))
            .go_to(TestItem(1.0), ONE_SECOND)
            .reverse();

        assert_eq!(keyframes.get(ZERO_DURATION), TestItem(1.0));
        assert_eq!(keyframes.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(keyframes.get(ONE_SECOND), TestItem(0.0));
    }
}
