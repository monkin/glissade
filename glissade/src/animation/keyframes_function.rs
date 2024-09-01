use crate::Time;
use std::fmt::Debug;

pub struct FunctionKeyframes<T, X: Time, F: Fn(X::Duration) -> T> {
    function: F,
    duration: X::Duration,
}

impl<T, X: Time, F: Fn(X::Duration) -> T> FunctionKeyframes<T, X, F> {
    pub fn new(function: F, duration: X::Duration) -> Self {
        Self { function, duration }
    }
}

impl<T, X: Time, F: Fn(X::Duration) -> T> crate::Keyframes<T, X> for FunctionKeyframes<T, X, F> {
    fn get(&self, offset: X::Duration) -> T {
        (self.function)(offset)
    }

    fn duration(&self) -> X::Duration {
        self.duration
    }

    fn is_finite(&self) -> bool {
        true
    }
}

impl<T, X: Time, F: Clone + Fn(X::Duration) -> T> Clone for FunctionKeyframes<T, X, F> {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
            duration: self.duration,
        }
    }
}

impl<T, X: Time, F: Copy + Fn(X::Duration) -> T> Copy for FunctionKeyframes<T, X, F> {}

impl<T, X, F> Debug for FunctionKeyframes<T, X, F>
where
    X: Time,
    X::Duration: Debug,
    F: Debug + Fn(X::Duration) -> T,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyframesFunction")
            .field("duration", &self.duration)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Keyframes;

    #[test]
    fn test_keyframes_function() {
        let keyframes: FunctionKeyframes<f32, f32, _> =
            FunctionKeyframes::new(|offset: f32| offset * 2.0, 1.0);
        assert_eq!(keyframes.get(0.0), 0.0);
        assert_eq!(keyframes.get(0.5), 1.0);
        assert_eq!(keyframes.get(1.0), 2.0);
        assert_eq!(keyframes.duration(), 1.0);
        assert!(keyframes.is_finite());
    }
}
