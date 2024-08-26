use crate::Time;
use std::fmt::Debug;

/// An animated value that changes over time.
/// It's a common trait for `Animation` and `Inertial`.
pub trait Animated<T, X: Time> {
    /// Get the value of the animation at a specific time.
    /// * `time` - The time to get the value of the animation, usually `Instant::now()`.
    fn get(&self, time: X) -> T;
    /// Check if the animation is finished at a specific time.
    fn is_finished(&self, time: X) -> bool;

    /// Map the animated value to another type.
    fn map<R, F: Fn(T) -> R>(self, map: F) -> AnimatedMap<T, X, Self, R, F>
    where
        Self: Sized,
    {
        AnimatedMap::new(self, map)
    }

    /// Join two animated values into a single animated tuple.
    /// The resulting animation will be finished when both animations are finished.
    fn join<T2, A2: Animated<T2, X>>(self, other: A2) -> AnimatedJoin<T, T2, X, Self, A2>
    where
        Self: Sized,
    {
        AnimatedJoin::new(self, other)
    }
}

impl<X: Time> Animated<(), X> for () {
    fn get(&self, _time: X) {}

    fn is_finished(&self, _time: X) -> bool {
        true
    }
}

impl<V, T, X: Time> Animated<(V,), X> for (T,)
where
    T: Animated<V, X>,
{
    fn get(&self, time: X) -> (V,) {
        (self.0.get(time),)
    }

    fn is_finished(&self, time: X) -> bool {
        self.0.is_finished(time)
    }
}

impl<V1, V2, T1, T2, X: Time> Animated<(V1, V2), X> for (T1, T2)
where
    T1: Animated<V1, X>,
    T2: Animated<V2, X>,
{
    fn get(&self, time: X) -> (V1, V2) {
        (self.0.get(time), self.1.get(time))
    }

    fn is_finished(&self, time: X) -> bool {
        self.0.is_finished(time) && self.1.is_finished(time)
    }
}

impl<V1, V2, V3, T1, T2, T3, X: Time> Animated<(V1, V2, V3), X> for (T1, T2, T3)
where
    T1: Animated<V1, X>,
    T2: Animated<V2, X>,
    T3: Animated<V3, X>,
{
    fn get(&self, time: X) -> (V1, V2, V3) {
        (self.0.get(time), self.1.get(time), self.2.get(time))
    }

    fn is_finished(&self, time: X) -> bool {
        self.0.is_finished(time) && self.1.is_finished(time) && self.2.is_finished(time)
    }
}

impl<V1, V2, V3, V4, T1, T2, T3, T4, X: Time> Animated<(V1, V2, V3, V4), X> for (T1, T2, T3, T4)
where
    T1: Animated<V1, X>,
    T2: Animated<V2, X>,
    T3: Animated<V3, X>,
    T4: Animated<V4, X>,
{
    fn get(&self, time: X) -> (V1, V2, V3, V4) {
        (
            self.0.get(time),
            self.1.get(time),
            self.2.get(time),
            self.3.get(time),
        )
    }

    fn is_finished(&self, time: X) -> bool {
        self.0.is_finished(time)
            && self.1.is_finished(time)
            && self.2.is_finished(time)
            && self.3.is_finished(time)
    }
}

// Animated implementation for arrays of animated items

impl<T: Clone + Copy + Default, X: Time, I: Animated<T, X>, const S: usize> Animated<[T; S], X>
    for [I; S]
{
    fn get(&self, time: X) -> [T; S] {
        let mut result: [T; S] = [Default::default(); S];
        for i in 0..S {
            result[i] = self[i].get(time);
        }
        result
    }

    fn is_finished(&self, time: X) -> bool {
        self.iter().all(|i| i.is_finished(time))
    }
}

/// Similar to `iter().map(...)`, but for animated values.
pub struct AnimatedMap<T, X: Time, A: Animated<T, X>, R, F: Fn(T) -> R> {
    animated: A,
    map: F,
    phantom: std::marker::PhantomData<(T, X)>,
}

impl<T, X: Time, A: Animated<T, X>, R, F: Fn(T) -> R> AnimatedMap<T, X, A, R, F> {
    pub fn new(animated: A, map: F) -> Self {
        Self {
            animated,
            map,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, A: Animated<T, X>, R, F: Fn(T) -> R> Animated<R, X>
    for AnimatedMap<T, X, A, R, F>
{
    fn get(&self, time: X) -> R {
        (self.map)(self.animated.get(time))
    }

    fn is_finished(&self, time: X) -> bool {
        self.animated.is_finished(time)
    }
}
impl<T, X: Time, A: Animated<T, X>, R, F: Fn(T) -> R> Clone for AnimatedMap<T, X, A, R, F>
where
    A: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            animated: self.animated.clone(),
            map: self.map.clone(),
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, A: Animated<T, X>, R, F: Fn(T) -> R> Debug for AnimatedMap<T, X, A, R, F>
where
    A: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimatedMap")
            .field("animated", &self.animated)
            .field("map", &"Fn(T) -> R")
            .finish()
    }
}

pub struct AnimatedJoin<T1, T2, X: Time, A1: Animated<T1, X>, A2: Animated<T2, X>> {
    animated1: A1,
    animated2: A2,
    phantom: std::marker::PhantomData<(T1, T2, X)>,
}

impl<T1, T2, X: Time, A1: Animated<T1, X>, A2: Animated<T2, X>> AnimatedJoin<T1, T2, X, A1, A2> {
    pub fn new(animated1: A1, animated2: A2) -> Self {
        Self {
            animated1,
            animated2,
            phantom: Default::default(),
        }
    }
}

impl<T1, T2, X: Time, A1: Animated<T1, X>, A2: Animated<T2, X>> Animated<(T1, T2), X>
    for AnimatedJoin<T1, T2, X, A1, A2>
{
    fn get(&self, time: X) -> (T1, T2) {
        (self.animated1.get(time), self.animated2.get(time))
    }

    fn is_finished(&self, time: X) -> bool {
        self.animated1.is_finished(time) && self.animated2.is_finished(time)
    }
}

impl<T1, T2, X: Time, A1: Animated<T1, X> + Clone, A2: Animated<T2, X> + Clone> Clone
    for AnimatedJoin<T1, T2, X, A1, A2>
{
    fn clone(&self) -> Self {
        Self {
            animated1: self.animated1.clone(),
            animated2: self.animated2.clone(),
            phantom: Default::default(),
        }
    }
}

impl<T1, T2, X: Time, A1: Animated<T1, X> + Copy, A2: Animated<T2, X> + Copy> Copy
    for AnimatedJoin<T1, T2, X, A1, A2>
{
}

impl<T1, T2, X: Time, A1: Animated<T1, X> + Debug, A2: Animated<T2, X> + Debug> Debug
    for AnimatedJoin<T1, T2, X, A1, A2>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimatedJoin")
            .field("animated1", &self.animated1)
            .field("animated2", &self.animated2)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate as glissade;
    use crate::Mix;
    use crate::{keyframes, Keyframes};

    #[derive(Clone, Copy, Debug, PartialEq, Mix)]
    struct TestItem(f32);

    #[test]
    fn animated_map() {
        let animated = keyframes(TestItem(0.0))
            .go_to(TestItem(1.0), 1.0)
            .run(0.0)
            .map(|item| (item.0 * 10.0) as i32);
        assert_eq!(animated.get(0.0), 0);
        assert_eq!(animated.get(0.5), 5);
        assert_eq!(animated.get(1.0), 10);
    }

    #[test]
    fn animated_join() {
        let animated1 = keyframes(TestItem(0.0))
            .go_to(TestItem(1.0), 1.0)
            .run(0.0)
            .map(|i| i.0);
        let animated2 = keyframes(TestItem(3.0))
            .go_to(TestItem(4.0), 2.0)
            .run(0.0)
            .map(|i| i.0);

        let animated = animated1.join(animated2);
        assert_eq!(animated.get(0.0), (0.0, 3.0));
        assert_eq!(animated.get(0.5), (0.5, 3.25));
        assert_eq!(animated.get(1.0), (1.0, 3.5));
        assert_eq!(animated.get(1.5), (1.0, 3.75));
        assert_eq!(animated.get(2.0), (1.0, 4.0));

        assert!(!animated.is_finished(0.0));
        assert!(!animated.is_finished(1.0));
        assert!(!animated.is_finished(1.5));
        assert!(animated.is_finished(2.0));
        assert!(animated.is_finished(3.0));
    }
}
