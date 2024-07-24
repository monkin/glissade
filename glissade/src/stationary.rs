use crate::{Animated, Time};

/// A value that doesn't change over time.
/// It allows using a static value as an always finished animation.
pub trait Stationary: Clone {}

impl<T: Stationary, X: Time> Animated<T, X> for T {
    fn get(&self, _time: X) -> T {
        self.clone()
    }

    fn is_finished(&self, _time: X) -> bool {
        true
    }
}

macro_rules! impl_stationary {
    ($($t:ty),*) => {
        $(impl Stationary for $t {})*
    };
}

impl_stationary!(
    f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, char, String,
    &str
);
