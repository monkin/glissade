use crate::Time;

/// An animated value that changes over time.
/// It's a common trait for `Animation` and `Inertial`.
pub trait Animated<T, X: Time> {
    /// Get the value of the animation at a specific time.
    /// * `time` - The time to get the value of the animation, usually `Instant::now()`.
    fn get(&self, time: X) -> T;
    /// Check if the animation is finished at a specific time.
    fn is_finished(&self, time: X) -> bool;
}
