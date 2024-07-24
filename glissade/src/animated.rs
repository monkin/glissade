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
