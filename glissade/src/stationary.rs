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

// implement `Static` for all numeric types
impl Stationary for f32 {}
impl Stationary for f64 {}
impl Stationary for i8 {}
impl Stationary for i16 {}
impl Stationary for i32 {}
impl Stationary for i64 {}
impl Stationary for i128 {}
impl Stationary for isize {}
impl Stationary for u8 {}
impl Stationary for u16 {}
impl Stationary for u32 {}
impl Stationary for u64 {}
impl Stationary for u128 {}
impl Stationary for usize {}

impl Stationary for bool {}
impl Stationary for char {}
impl Stationary for String {}
impl Stationary for () {}
impl Stationary for &str {}
