/// Time trait should be implemented for types that represent animation time.
/// It's implemented for `f32`, `f64`, `std::time::Instant`, and `std::time::SystemTime` by default.
/// You can implement it for your own types.
pub trait Time: PartialEq + PartialOrd + Clone + Copy {
    /// Positive time difference
    type Duration: Default + PartialEq + PartialOrd + Clone + Copy;

    /// Panics if `self < earlier`
    fn since(self, earlier: Self) -> Self::Duration;
    fn advance(self, duration: Self::Duration) -> Self;

    fn duration_as_f32(duration: Self::Duration) -> f32;
    fn duration_sum(duration: Self::Duration, other: Self::Duration) -> Self::Duration;
    fn duration_diff(duration: Self::Duration, other: Self::Duration) -> Self::Duration;
    fn duration_scale(duration: Self::Duration, scale: f32) -> Self::Duration;
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

    fn duration_as_f32(duration: f32) -> f32 {
        duration
    }

    fn duration_sum(duration: f32, other: f32) -> f32 {
        duration + other
    }

    fn duration_diff(duration: f32, other: f32) -> f32 {
        if duration < other {
            panic!("Time::sub_duration: duration < other");
        }
        duration - other
    }

    fn duration_scale(duration: f32, scale: f32) -> f32 {
        if scale < 0.0 {
            panic!("Time::scale_duration: scale < 0.0");
        }
        duration * scale
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

    fn duration_as_f32(duration: f64) -> f32 {
        duration as f32
    }

    fn duration_sum(duration: f64, other: f64) -> f64 {
        duration + other
    }

    fn duration_diff(duration: f64, other: f64) -> f64 {
        if duration < other {
            panic!("Time::sub_duration: duration < other");
        }
        duration - other
    }

    fn duration_scale(duration: f64, scale: f32) -> f64 {
        if scale < 0.0 {
            panic!("Time::scale_duration: scale < 0.0");
        }
        duration * scale as f64
    }
}
