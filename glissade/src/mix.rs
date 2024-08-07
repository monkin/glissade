/// Mix trait for linear interpolation between two values.
pub trait Mix {
    /// Linearly interpolate between two values using a factor `t` in the range [0, 1].
    fn mix(self, other: Self, t: f32) -> Self;
}

impl Mix for f32 {
    fn mix(self, other: f32, t: f32) -> f32 {
        self + (other - self) * t
    }
}

impl Mix for f64 {
    fn mix(self, other: f64, t: f32) -> f64 {
        self + (other - self) * t as f64
    }
}

impl<T> Mix for Option<T>
where
    T: Mix,
{
    fn mix(self, other: Self, t: f32) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.mix(b, t)),
            (Some(a), None) => {
                if t > 0.5 {
                    None
                } else {
                    Some(a)
                }
            }
            (None, Some(b)) => {
                if t > 0.5 {
                    Some(b)
                } else {
                    None
                }
            }
            (None, None) => None,
        }
    }
}

impl Mix for bool {
    fn mix(self, other: bool, t: f32) -> bool {
        if t <= 0.5 {
            self
        } else {
            other
        }
    }
}

impl Mix for i8 {
    fn mix(self, other: i8, t: f32) -> i8 {
        (self as f32).mix(other as f32, t).round() as i8
    }
}

impl Mix for u8 {
    fn mix(self, other: u8, t: f32) -> u8 {
        (self as f32).mix(other as f32, t).round() as u8
    }
}

impl Mix for i16 {
    fn mix(self, other: i16, t: f32) -> i16 {
        (self as f32).mix(other as f32, t).round() as i16
    }
}

impl Mix for u16 {
    fn mix(self, other: u16, t: f32) -> u16 {
        (self as f32).mix(other as f32, t).round() as u16
    }
}

impl Mix for i32 {
    fn mix(self, other: i32, t: f32) -> i32 {
        (self as f32).mix(other as f32, t).round() as i32
    }
}

impl Mix for u32 {
    fn mix(self, other: u32, t: f32) -> u32 {
        (self as f32).mix(other as f32, t).round() as u32
    }
}

impl Mix for i64 {
    fn mix(self, other: i64, t: f32) -> i64 {
        (self as f64).mix(other as f64, t).round() as i64
    }
}

impl Mix for u64 {
    fn mix(self, other: u64, t: f32) -> u64 {
        (self as f64).mix(other as f64, t).round() as u64
    }
}

impl Mix for isize {
    fn mix(self, other: isize, t: f32) -> isize {
        (self as f64).mix(other as f64, t).round() as isize
    }
}

impl Mix for usize {
    fn mix(self, other: usize, t: f32) -> usize {
        (self as f64).mix(other as f64, t).round() as usize
    }
}

impl<T1, T2> Mix for (T1, T2)
where
    T1: Mix,
    T2: Mix,
{
    fn mix(self, other: Self, t: f32) -> (T1, T2) {
        (self.0.mix(other.0, t), self.1.mix(other.1, t))
    }
}

impl<T1, T2, T3> Mix for (T1, T2, T3)
where
    T1: Mix,
    T2: Mix,
    T3: Mix,
{
    fn mix(self, other: Self, t: f32) -> (T1, T2, T3) {
        (
            self.0.mix(other.0, t),
            self.1.mix(other.1, t),
            self.2.mix(other.2, t),
        )
    }
}

impl<T1, T2, T3, T4> Mix for (T1, T2, T3, T4)
where
    T1: Mix,
    T2: Mix,
    T3: Mix,
    T4: Mix,
{
    fn mix(self, other: Self, t: f32) -> (T1, T2, T3, T4) {
        (
            self.0.mix(other.0, t),
            self.1.mix(other.1, t),
            self.2.mix(other.2, t),
            self.3.mix(other.3, t),
        )
    }
}

impl<T: Mix + Default + Copy, const N: usize> Mix for [T; N] {
    fn mix(self, other: Self, t: f32) -> Self {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self[i].mix(other[i], t);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1.0f64.mix(2.0, 0.5), 1.5f64);
        assert_eq!(1.0f32.mix(2.0, 0.5), 1.5f32);
        assert_eq!((1.0, 2.0).mix((2.0, 3.0), 0.5), (1.5, 2.5));
        assert_eq!((1.0, 2.0, 3.0).mix((2.0, 3.0, 4.0), 0.5), (1.5, 2.5, 3.5));
        assert_eq!(
            (1.0, 2.0, 3.0, 4.0).mix((2.0, 3.0, 4.0, 5.0), 0.5),
            (1.5, 2.5, 3.5, 4.5)
        );
    }

    #[test]
    fn test_mix_integer() {
        assert_eq!(1i8.mix(3, 0.5), 2);
        assert_eq!(1u8.mix(3, 0.5), 2);
        assert_eq!(1i16.mix(3, 0.5), 2);
        assert_eq!(1u16.mix(3, 0.5), 2);
        assert_eq!(1i32.mix(3, 0.5), 2);
        assert_eq!(1u32.mix(3, 0.5), 2);
        assert_eq!(1i64.mix(3, 0.5), 2);
        assert_eq!(1u64.mix(3, 0.5), 2);
        assert_eq!(1isize.mix(3, 0.5), 2);
        assert_eq!(1usize.mix(3, 0.5), 2);
    }

    #[test]
    fn test_slice_mix() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert_eq!(a.mix(b, 0.5), [2.5, 3.5, 4.5]);
    }

    #[test]
    fn test_option_mix() {
        assert_eq!(Some(1).mix(Some(3), 0.5), Some(2));
        assert_eq!(Some(1).mix(None, 0.25), Some(1));
        assert_eq!(None.mix(Some(2), 0.25), None);
        assert_eq!(Some(1).mix(None, 0.75), None);
        assert_eq!(None.mix(Some(2), 0.75), Some(2));

        let v1: Option<f32> = None;
        let v2: Option<f32> = None;
        assert_eq!(v1.mix(v2, 0.5), None);
    }
}
