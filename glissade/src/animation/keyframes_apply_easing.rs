use crate::{Easing, Keyframes, Time};
use std::fmt::Debug;
use std::marker::PhantomData;

/// Apply easing to keyframes.
pub struct ApplyEasingKeyframes<T, X: Time, K: Keyframes<T, X>> {
    keyframes: K,
    easing: Easing,
    phantom: PhantomData<(T, X)>,
}

impl<T, X: Time, K: Keyframes<T, X>> ApplyEasingKeyframes<T, X, K> {
    pub fn new(keyframes: K, easing: Easing) -> Self {
        assert!(keyframes.is_finite());
        Self {
            keyframes,
            easing,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, K: Keyframes<T, X>> Keyframes<T, X> for ApplyEasingKeyframes<T, X, K> {
    fn get(&self, offset: X::Duration) -> T {
        let t = X::duration_as_f32(offset) / X::duration_as_f32(self.keyframes.duration());
        let t = self.easing.ease(t).clamp(0.0, 1.0);
        let offset = X::duration_scale(self.keyframes.duration(), t);
        self.keyframes.get(offset)
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration()
    }

    fn is_finite(&self) -> bool {
        self.keyframes.is_finite()
    }
}

impl<T, X: Time, K: Keyframes<T, X> + Clone> Clone for ApplyEasingKeyframes<T, X, K> {
    fn clone(&self) -> Self {
        Self {
            keyframes: self.keyframes.clone(),
            easing: self.easing.clone(),
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, K: Keyframes<T, X> + Debug> Debug for ApplyEasingKeyframes<T, X, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplyEasingKeyframes")
            .field("keyframes", &self.keyframes)
            .field("easing", &self.easing)
            .finish()
    }
}

impl<T, X: Time, K: Keyframes<T, X> + PartialEq> PartialEq for ApplyEasingKeyframes<T, X, K> {
    fn eq(&self, other: &Self) -> bool {
        self.keyframes == other.keyframes && self.easing == other.easing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{keyframes, Animated};

    #[test]
    fn test_apply_easing_keyframes() {
        let keyframes = keyframes::line(0.0, 4.0, 1.0)
            .go_to(12.0, 1.0)
            .apply_easing(Easing::QuadraticInOut)
            .run(0.0);

        assert_eq!(keyframes.get(0.0), 0.0);
        assert_eq!(keyframes.get(0.5), 1.0);
        assert_eq!(keyframes.get(1.0), 4.0);
        assert_eq!(keyframes.get(1.5), 10.0);
        assert_eq!(keyframes.get(2.0), 12.0);
    }
}
