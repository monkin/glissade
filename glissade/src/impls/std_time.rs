use crate::Time;

impl Time for std::time::Instant {
    type Duration = std::time::Duration;
    fn since(self, earlier: Self) -> Self::Duration {
        self.duration_since(earlier)
    }

    fn advance(self, duration: Self::Duration) -> Self {
        self + duration
    }

    fn duration_as_f32(duration: Self::Duration) -> f32 {
        duration.as_secs_f32()
    }

    fn duration_sum(duration: Self::Duration, other: Self::Duration) -> Self::Duration {
        duration + other
    }

    fn duration_diff(duration: Self::Duration, other: Self::Duration) -> Self::Duration {
        duration - other
    }

    fn duration_scale(duration: Self::Duration, scale: f32) -> Self::Duration {
        duration.mul_f32(scale)
    }
}

impl Time for std::time::SystemTime {
    type Duration = std::time::Duration;
    fn since(self, earlier: Self) -> Self::Duration {
        self.duration_since(earlier).unwrap()
    }

    fn advance(self, duration: Self::Duration) -> Self {
        self + duration
    }

    fn duration_as_f32(duration: Self::Duration) -> f32 {
        duration.as_secs_f32()
    }

    fn duration_sum(duration: Self::Duration, other: Self::Duration) -> Self::Duration {
        duration + other
    }

    fn duration_diff(duration: Self::Duration, other: Self::Duration) -> Self::Duration {
        duration - other
    }

    fn duration_scale(duration: Self::Duration, scale: f32) -> Self::Duration {
        duration.mul_f32(scale)
    }
}
