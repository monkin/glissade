use crate::{Keyframes, Time};
use std::marker::PhantomData;

pub struct MapKeyframes<T, R, X, K, F>
where
    X: Time,
    K: Keyframes<T, X>,
    F: Fn(T) -> R,
{
    keyframes: K,
    map: F,
    phantom_data: PhantomData<(T, R, X)>,
}

impl<T, R, X, K, F> MapKeyframes<T, R, X, K, F>
where
    X: Time,
    K: Keyframes<T, X>,
    F: Fn(T) -> R,
{
    pub fn new(keyframes: K, map: F) -> Self {
        Self {
            keyframes,
            map,
            phantom_data: Default::default(),
        }
    }
}

impl<T, R, X, K, F> Keyframes<R, X> for MapKeyframes<T, R, X, K, F>
where
    X: Time,
    K: Keyframes<T, X>,
    F: Fn(T) -> R,
{
    fn get(&self, offset: X::Duration) -> R {
        (self.map)(self.keyframes.get(offset))
    }

    fn duration(&self) -> X::Duration {
        self.keyframes.duration()
    }

    fn is_finite(&self) -> bool {
        self.keyframes.is_finite()
    }
}
