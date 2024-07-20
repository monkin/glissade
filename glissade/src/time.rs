/// Positive time difference
pub trait TimeDiff: Default + PartialEq + PartialOrd + Clone + Copy {
    fn as_f32(self) -> f32;
    fn add(self, other: Self) -> Self;

    /// Panics if `self < other`
    fn sub(self, other: Self) -> Self;

    /// Panics if `scale < 0.0`
    fn scale(self, scale: f32) -> Self;
}

/// Time trait should be implemented for types that represent animation time.
/// It's implemented for `f32`, `f64`, `std::time::Instant`, and `std::time::SystemTime` by default.
/// You can implement it for your own types.
pub trait Time: PartialEq + PartialOrd + Clone + Copy {
    type Duration: TimeDiff;

    /// Panics if `self < earlier`
    fn since(self, earlier: Self) -> Self::Duration;
    fn advance(self, duration: Self::Duration) -> Self;
}

impl Time for f32 {
    type Duration = f32;
    fn since(self, earlier: f32) -> f32 {
        if self < earlier {
            panic!("Time::since: self < earlier");
        }

        self - earlier
    }

    fn advance(self, duration: f32) -> f32 {
        self + duration
    }
}

impl TimeDiff for f32 {
    fn as_f32(self) -> f32 {
        self
    }

    fn add(self, other: f32) -> f32 {
        self + other
    }

    fn sub(self, other: f32) -> f32 {
        if self < other {
            panic!("TimeDiff::sub: self < other");
        }
        self - other
    }

    fn scale(self, scale: f32) -> f32 {
        if scale < 0.0 {
            panic!("TimeDiff::scale: scale < 0.0");
        }
        self * scale
    }
}

impl Time for f64 {
    type Duration = f64;
    fn since(self, earlier: f64) -> f64 {
        if self < earlier {
            panic!("Time::since: self < earlier");
        }
        self - earlier
    }

    fn advance(self, duration: f64) -> f64 {
        self + duration
    }
}

impl TimeDiff for f64 {
    fn as_f32(self) -> f32 {
        self as f32
    }

    fn add(self, other: f64) -> f64 {
        self + other
    }

    fn sub(self, other: f64) -> f64 {
        if self < other {
            panic!("TimeDiff::sub: self < other");
        }
        self - other
    }

    fn scale(self, scale: f32) -> f64 {
        if scale < 0.0 {
            panic!("TimeDiff::scale: scale < 0.0");
        }
        self * f64::from(scale)
    }
}
