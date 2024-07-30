pub trait Distance {
    /// Calculate the distance between two values.
    fn distance(self, other: Self) -> f32;
}

impl Distance for f32 {
    fn distance(self, other: f32) -> f32 {
        (self - other).abs()
    }
}

impl Distance for f64 {
    fn distance(self, other: f64) -> f32 {
        (self - other).abs() as f32
    }
}

macro_rules! impl_distance_for_int {
    ($($t:ty),*) => {
        $(
            impl Distance for $t {
                fn distance(self, other: $t) -> f32 {
                    self.abs_diff(other) as f32
                }
            }
        )*
    };
}

impl_distance_for_int!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

impl Distance for bool {
    fn distance(self, other: bool) -> f32 {
        if self == other {
            0.0
        } else {
            1.0
        }
    }
}

impl Distance for () {
    fn distance(self, _: ()) -> f32 {
        0.0
    }
}

impl<T: Distance> Distance for (T,) {
    fn distance(self, other: Self) -> f32 {
        self.0.distance(other.0)
    }
}

impl<T1, T2> Distance for (T1, T2)
where
    T1: Distance,
    T2: Distance,
{
    fn distance(self, other: Self) -> f32 {
        let v1 = self.0.distance(other.0);
        let v2 = self.1.distance(other.1);
        (v1 * v1 + v2 * v2).sqrt()
    }
}

impl<T1, T2, T3> Distance for (T1, T2, T3)
where
    T1: Distance,
    T2: Distance,
    T3: Distance,
{
    fn distance(self, other: Self) -> f32 {
        let v1 = self.0.distance(other.0);
        let v2 = self.1.distance(other.1);
        let v3 = self.2.distance(other.2);
        (v1 * v1 + v2 * v2 + v3 * v3).sqrt()
    }
}

impl<T1, T2, T3, T4> Distance for (T1, T2, T3, T4)
where
    T1: Distance,
    T2: Distance,
    T3: Distance,
    T4: Distance,
{
    fn distance(self, other: Self) -> f32 {
        let v1 = self.0.distance(other.0);
        let v2 = self.1.distance(other.1);
        let v3 = self.2.distance(other.2);
        let v4 = self.3.distance(other.3);
        (v1 * v1 + v2 * v2 + v3 * v3 + v4 * v4).sqrt()
    }
}

impl<T: Distance + Clone, const N: usize> Distance for [T; N] {
    fn distance(self, other: Self) -> f32 {
        self.into_iter()
            .zip(other)
            .map(|(a, b)| a.distance(b))
            .fold(0.0, |acc, x| acc + x * x)
            .sqrt()
    }
}
