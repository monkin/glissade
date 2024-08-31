use crate::{Keyframes, Time, TimeDiff};
use std::fmt::Debug;
use std::marker::PhantomData;

/// An animation that scales the time of keyframes.
pub struct ScaleKeyframes<T, X: Time, S: Keyframes<T, X>> {
    keyframes: S,
    scale: f32,
    phantom: PhantomData<(T, X)>,
}

impl<T, X: Time, S: Keyframes<T, X> + Debug> Debug for ScaleKeyframes<T, X, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScaleKeyframes")
            .field("keyframes", &self.keyframes)
            .field("scale", &self.scale)
            .finish()
    }
}

impl<T, X: Time, S: Keyframes<T, X> + PartialEq> PartialEq for ScaleKeyframes<T, X, S> {
    fn eq(&self, other: &Self) -> bool {
        self.keyframes == other.keyframes && self.scale == other.scale
    }
}

impl<T, X: Time, S: Keyframes<T, X>> ScaleKeyframes<T, X, S> {
    pub fn new(keyframes: S, scale: f32) -> Self {
        Self {
            keyframes,
            scale,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, S: Keyframes<T, X>> Keyframes<T, X> for ScaleKeyframes<T, X, S> {
    fn get(&self, offset: X::Duration) -> T {
        self.keyframes.get(offset.scale(self.scale))
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration().scale(self.scale)
    }

    fn is_finite(&self) -> bool {
        self.keyframes.is_finite()
    }
}

impl<T, X: Time, S: Keyframes<T, X> + Clone> Clone for ScaleKeyframes<T, X, S> {
    fn clone(&self) -> Self {
        Self {
            keyframes: self.keyframes.clone(),
            scale: self.scale,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, S: Keyframes<T, X> + Copy> Copy for ScaleKeyframes<T, X, S> {}
