use crate::{Keyframes, Time};
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct SliceKeyframes<T, X: Time, K: Keyframes<T, X>> {
    keyframes: K,
    range: (X::Duration, X::Duration),
    phantom: PhantomData<T>,
}

impl<T, X: Time, K: Keyframes<T, X>> SliceKeyframes<T, X, K> {
    pub fn new(keyframes: K, range: (X::Duration, X::Duration)) -> Self {
        Self {
            keyframes,
            range,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, K: Keyframes<T, X>> Keyframes<T, X> for SliceKeyframes<T, X, K> {
    fn get(&self, offset: X::Duration) -> T {
        let offset = X::duration_sum(offset, self.range.0);
        let offset = if offset < self.range.0 {
            self.range.0
        } else if offset > self.range.1 {
            self.range.1
        } else {
            offset
        };
        self.keyframes.get(offset)
    }

    fn duration(&self) -> X::Duration {
        X::duration_diff(self.range.1, self.range.0)
    }

    fn is_finite(&self) -> bool {
        true
    }
}

impl<T, X, K> Debug for SliceKeyframes<T, X, K>
where
    X: Time,
    X::Duration: Debug,
    K: Keyframes<T, X> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SliceKeyframes")
            .field("keyframes", &self.keyframes)
            .field("range", &self.range)
            .finish()
    }
}

impl<T, X: Time, K: Keyframes<T, X> + Clone> Clone for SliceKeyframes<T, X, K> {
    fn clone(&self) -> Self {
        Self {
            keyframes: self.keyframes.clone(),
            range: self.range,
            phantom: Default::default(),
        }
    }
}

impl<T, X: Time, K: Keyframes<T, X> + Copy> Copy for SliceKeyframes<T, X, K> {}

impl<T, X: Time, K: Keyframes<T, X> + PartialEq> PartialEq for SliceKeyframes<T, X, K> {
    fn eq(&self, other: &Self) -> bool {
        self.keyframes == other.keyframes && self.range == other.range
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::keyframes_linear::LinearKeyframes;

    #[test]
    fn test_slice_keyframes() {
        let keyframes: LinearKeyframes<f32, f64> = LinearKeyframes::new(1.0, 5.0, 4.0);
        let keyframes = keyframes.slice(1.0, 3.0);
        assert_eq!(keyframes.get(0.0), 2.0);
        assert_eq!(keyframes.get(1.0), 3.0);
        assert_eq!(keyframes.get(2.0), 4.0);
        assert_eq!(keyframes.get(3.0), 4.0);
        assert_eq!(keyframes.get(4.0), 4.0);
        assert_eq!(keyframes.get(5.0), 4.0);
        assert_eq!(keyframes.duration(), 2.0);
    }
}
