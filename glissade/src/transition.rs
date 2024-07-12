use crate::animation::Animation;
use crate::easing::Easing;
use crate::transition_item::TransitionItem;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::{Duration, SystemTime};

/// A transition of a value over time. It works like an animation template.
/// A good point to start building transition is the [`transition`] function.
pub trait Transition<T: Clone + Debug + Sized>: Sized {
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
    fn stay(self, duration: Duration) -> SequentialTransition<T, Self, NoneTransition<T>>
    where
        Self: Sized,
        T: TransitionItem,
    {
        let end_value = self.end_value();
        SequentialTransition::new(self, NoneTransition::new(end_value, duration))
    }

    /// Create a transition that linearly interpolates between the end value and the target value.
    fn go_to(
        self,
        target: T,
        duration: Duration,
    ) -> SequentialTransition<T, Self, LinearTransition<T>>
    where
        Self: Sized,
        T: TransitionItem,
    {
        let end_value = self.end_value();
        SequentialTransition::new(self, LinearTransition::new(end_value, target, duration))
    }

    /// Create a transition that eases between the end value and the target value.
    fn ease_to(
        self,
        target: T,
        duration: Duration,
        easing: Easing,
    ) -> SequentialTransition<T, Self, EasingTransition<T>>
    where
        Self: Sized,
        T: TransitionItem,
    {
        let end_value = self.end_value();
        SequentialTransition::new(
            self,
            EasingTransition::new(end_value, target, duration, easing),
        )
    }

    /// Create a transition that repeats the given transition indefinitely.
    fn repeat(self) -> RepeatTransition<T, Self>
    where
        Self: Sized,
    {
        RepeatTransition::new(self)
    }

    /// Create a transition that repeats the given transition n times.
    fn repeat_n(self, n: f32) -> RepeatNTransition<T, Self>
    where
        Self: Sized,
    {
        RepeatNTransition::new(self, n)
    }

    /// Create a transition that reverses the given transition.
    fn reverse(self) -> ReverseTransition<T, Self>
    where
        Self: Sized,
    {
        ReverseTransition::new(self)
    }

    /// Scale the time of the transition by the given factor.
    fn scale(self, scale: f32) -> ScaleTransition<T, Self>
    where
        Self: Sized,
    {
        ScaleTransition::new(self, scale)
    }

    /// Scale the time of the transition to the given duration.
    fn scale_to(self, new_duration: Duration) -> ScaleTransition<T, Self>
    where
        Self: Sized,
    {
        let scale = if self.duration().is_zero() {
            1.0
        } else {
            new_duration.as_secs_f32() / self.duration().as_secs_f32()
        };

        ScaleTransition::new(self, scale)
    }

    /// Similar to Vec::map, creates a transition that applies the given function to the value at each point in time.
    fn map<F: Fn(T) -> T>(self, map: F) -> MapTransition<T, Self, F>
    where
        Self: Sized,
    {
        MapTransition::new(self, map)
    }

    /// Concatenate two transitions.
    fn then<S: Transition<T>>(self, other: S) -> SequentialTransition<T, Self, S>
    where
        Self: Sized,
    {
        SequentialTransition::new(self, other)
    }

    /// Invert the direction of the transition.
    fn invert(self) -> InvertedTransition<T, Self>
    where
        Self: Sized,
    {
        InvertedTransition::new(self)
    }

    /// Run the transition at a specific time.
    /// * `start_time` - The time to start the transition, usually `SystemTime::now()`.
    fn run(self, start_time: SystemTime) -> Animation<T, Self> {
        Animation::start(self, start_time)
    }
}

/// Start transition constructing with this function. It receives the initial value.
/// * `value` - The value to start at.
///
/// See [`Transition`] trait methods for more options of constructing transitions.
///
/// # Examples
///
/// ```
/// use glissade::{transition, Transition};
/// use std::time::Duration;
///
/// let transition = transition(5.0)
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
pub fn transition<T: TransitionItem>(value: T) -> NoneTransition<T> {
    NoneTransition::new(value, Duration::default())
}

//----------------------------------------------------------------
// NoneTransition

/// A transition that stays at a single value.
#[derive(Clone, Debug)]
pub struct NoneTransition<T: TransitionItem> {
    value: T,
    duration: Duration,
}

impl<T: TransitionItem> NoneTransition<T> {
    pub fn new(value: T, duration: Duration) -> Self {
        Self { value, duration }
    }
}

impl<T: TransitionItem> Transition<T> for NoneTransition<T> {
    fn get(&self, _offset: Duration) -> T {
        self.value.clone()
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl<T: TransitionItem + Copy> Copy for NoneTransition<T> {}

//----------------------------------------------------------------
// LinearTransition

/// A transition that linearly interpolates between two values.
#[derive(Clone, Debug)]
pub struct LinearTransition<T: TransitionItem> {
    v1: T,
    v2: T,
    duration: Duration,
}

impl<T: TransitionItem> LinearTransition<T> {
    pub fn new(v1: T, v2: T, duration: Duration) -> Self {
        Self { v1, v2, duration }
    }
}

impl<T: TransitionItem> Transition<T> for LinearTransition<T> {
    fn get(&self, offset: Duration) -> T {
        let t = offset.as_millis() as f32 / self.duration.as_millis() as f32;
        self.v1.clone().mix(self.v2.clone(), t)
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl<T: TransitionItem + Copy> Copy for LinearTransition<T> {}

//----------------------------------------------------------------
// SequentialTransition

/// A sequence of two transitions.
#[derive(Clone, Debug)]
pub struct SequentialTransition<T: Clone + Debug + Sized, S1: Transition<T>, S2: Transition<T>> {
    t1: S1,
    t2: S2,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug + Sized, S1: Transition<T>, S2: Transition<T>> Transition<T>
    for SequentialTransition<T, S1, S2>
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

impl<T: Clone + Debug + Sized, S1: Transition<T>, S2: Transition<T>>
    SequentialTransition<T, S1, S2>
{
    pub fn new(t1: S1, t2: S2) -> Self {
        Self {
            t1,
            t2,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Debug + Sized + Copy, S1: Transition<T> + Copy, S2: Transition<T> + Copy> Copy
    for SequentialTransition<T, S1, S2>
{
}

//----------------------------------------------------------------
// EasingTransition

/// A transition that eases between two values.
#[derive(Clone, Debug)]
pub struct EasingTransition<T: TransitionItem> {
    v1: T,
    v2: T,
    duration: Duration,
    easing: Easing,
}

impl<T: TransitionItem> EasingTransition<T> {
    pub fn new(v1: T, v2: T, duration: Duration, easing: Easing) -> Self {
        Self {
            v1,
            v2,
            duration,
            easing,
        }
    }
}

impl<T: TransitionItem> Transition<T> for EasingTransition<T> {
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
// RepeatTransition

/// A transition that repeats another transition indefinitely.
#[derive(Clone, Debug)]
pub struct RepeatTransition<T: Clone + Debug + Sized, S: Transition<T>> {
    transition: S,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug + Sized, S: Transition<T>> RepeatTransition<T, S> {
    pub fn new(transition: S) -> Self {
        Self {
            transition,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Debug + Sized, S: Transition<T>> Transition<T> for RepeatTransition<T, S> {
    fn get(&self, offset: Duration) -> T {
        self.transition.get(Duration::from_millis(
            offset.as_millis() as u64 % self.transition.duration().as_millis() as u64,
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

impl<T: Clone + Debug + Sized + Copy, S: Transition<T> + Copy> Copy for RepeatTransition<T, S> {}

//----------------------------------------------------------------
// RepeatNTransition

/// A transition that repeats another transition n times.
#[derive(Clone, Debug)]
pub struct RepeatNTransition<T: Clone + Debug + Sized, S: Transition<T>> {
    transition: S,
    n: f32,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug + Sized, S: Transition<T>> RepeatNTransition<T, S> {
    pub fn new(transition: S, n: f32) -> Self {
        Self {
            transition,
            n,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Debug + Sized, S: Transition<T>> Transition<T> for RepeatNTransition<T, S> {
    fn get(&self, offset: Duration) -> T {
        let n = (offset.as_millis() as f64 / self.transition.duration().as_millis() as f64) as f32;
        if n < self.n {
            self.transition.get(Duration::from_millis(
                offset.as_millis() as u64 % self.transition.duration().as_millis() as u64,
            ))
        } else {
            self.transition.end_value()
        }
    }

    fn duration(&self) -> Duration {
        Duration::from_millis((self.transition.duration().as_millis() as f32 * self.n) as u64)
    }
}

impl<T: Clone + Debug + Sized + Copy, S: Transition<T> + Copy> Copy for RepeatNTransition<T, S> {}

//----------------------------------------------------------------
// ReverseTransition

/// A transition that reverses the value of another transition.
#[derive(Clone, Debug)]
pub struct ReverseTransition<T: Clone + Debug + Sized, S: Transition<T>> {
    transition: S,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug + Sized, S: Transition<T>> ReverseTransition<T, S> {
    pub fn new(transition: S) -> Self {
        Self {
            transition,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Debug + Sized, S: Transition<T>> Transition<T> for ReverseTransition<T, S> {
    fn get(&self, time: Duration) -> T {
        self.transition.get(self.transition.duration() - time)
    }

    fn duration(&self) -> Duration {
        self.transition.duration()
    }
}

impl<T: Clone + Debug + Sized + Copy, S: Transition<T> + Copy> Copy for ReverseTransition<T, S> {}

//----------------------------------------------------------------
// MapTransition

/// A transition that maps the value of another transition.
#[derive(Clone, Debug)]
pub struct MapTransition<T: Clone + Debug + Sized, S: Transition<T>, F: Fn(T) -> T> {
    transition: S,
    map: F,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug + Sized, S: Transition<T>, F: Fn(T) -> T> MapTransition<T, S, F> {
    pub fn new(transition: S, map: F) -> Self {
        Self {
            transition,
            map,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Debug + Sized, S: Transition<T>, F: Fn(T) -> T> Transition<T>
    for MapTransition<T, S, F>
{
    fn get(&self, time: Duration) -> T {
        (self.map)(self.transition.get(time))
    }

    fn duration(&self) -> Duration {
        self.transition.duration()
    }
}

impl<T: Clone + Debug + Sized + Copy, S: Transition<T> + Copy, F: Copy + Fn(T) -> T> Copy
    for MapTransition<T, S, F>
{
}

//----------------------------------------------------------------
// ScaleTransition

/// A transition that scales the time of another transition.
#[derive(Clone, Debug)]
pub struct ScaleTransition<T: Clone + Debug + Sized, S: Transition<T>> {
    transition: S,
    scale: f32,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug + Sized, S: Transition<T>> ScaleTransition<T, S> {
    pub fn new(transition: S, scale: f32) -> Self {
        Self {
            transition,
            scale,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Debug + Sized, S: Transition<T>> Transition<T> for ScaleTransition<T, S> {
    fn get(&self, time: Duration) -> T {
        self.transition.get(Duration::from_millis(
            (time.as_millis() as f32 * self.scale) as u64,
        ))
    }

    fn duration(&self) -> Duration {
        Duration::from_millis((self.transition.duration().as_millis() as f32 * self.scale) as u64)
    }
}

impl<T: Clone + Debug + Sized + Copy, S: Transition<T> + Copy> Copy for ScaleTransition<T, S> {}

//----------------------------------------------------------------
// InvertedTransition

/// A transition that inverts the direction of another transition.
#[derive(Clone, Debug)]
pub struct InvertedTransition<T: Clone + Debug + Sized, S: Transition<T>> {
    transition: S,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug + Sized, S: Transition<T>> InvertedTransition<T, S> {
    pub fn new(transition: S) -> Self {
        Self {
            transition,
            phantom: Default::default(),
        }
    }
}

impl<T: Clone + Debug + Sized, S: Transition<T>> Transition<T> for InvertedTransition<T, S> {
    fn get(&self, time: Duration) -> T {
        self.transition.get(self.transition.duration() - time)
    }

    fn duration(&self) -> Duration {
        self.transition.duration()
    }
}

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
        let transition = NoneTransition::new(TestItem(0.0), Duration::from_secs(1));
        assert_eq!(transition.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(transition.get(HALF_SECOND), TestItem(0.0));
        assert_eq!(transition.get(ONE_SECOND), TestItem(0.0));
    }

    #[test]
    fn linear_transition() {
        let transition = LinearTransition::new(TestItem(0.0), TestItem(1.0), ONE_SECOND);
        assert_eq!(transition.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(transition.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(transition.get(ONE_SECOND), TestItem(1.0));
    }

    #[test]
    fn sequential_transition() {
        let transition = SequentialTransition::new(
            LinearTransition::new(TestItem(0.0), TestItem(1.0), ONE_SECOND),
            LinearTransition::new(TestItem(1.0), TestItem(0.0), ONE_SECOND),
        );
        assert_eq!(transition.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(transition.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(transition.get(ONE_SECOND), TestItem(1.0));
        assert_eq!(transition.get(ONE_AND_HALF_SECONDS), TestItem(0.5));
        assert_eq!(transition.get(TWO_SECONDS), TestItem(0.0));
    }

    #[test]
    fn easing_transition() {
        let transition = EasingTransition::new(
            TestItem(0.0),
            TestItem(1.0),
            ONE_SECOND,
            Easing::QuadraticIn,
        );
        assert_eq!(transition.get(ZERO_DURATION), TestItem(0.0));
        assert_eq!(transition.get(HALF_SECOND), TestItem(0.25));
        assert_eq!(transition.get(ONE_SECOND), TestItem(1.0));
    }

    #[test]
    fn inverted_transition() {
        let transition = transition(TestItem(0.0))
            .go_to(TestItem(1.0), ONE_SECOND)
            .invert();

        assert_eq!(transition.get(ZERO_DURATION), TestItem(1.0));
        assert_eq!(transition.get(HALF_SECOND), TestItem(0.5));
        assert_eq!(transition.get(ONE_SECOND), TestItem(0.0));
    }
}
