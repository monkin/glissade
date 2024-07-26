use crate::{Time, TimeDiff};

impl Time for web_time::Instant {
    type Duration = web_time::Duration;
    fn since(self, earlier: Self) -> Self::Duration {
        self.duration_since(earlier)
    }

    fn advance(self, duration: Self::Duration) -> Self {
        self + duration
    }
}

impl Time for web_time::SystemTime {
    type Duration = web_time::Duration;
    fn since(self, earlier: Self) -> Self::Duration {
        self.duration_since(earlier).unwrap()
    }

    fn advance(self, duration: Self::Duration) -> Self {
        self + duration
    }
}

impl TimeDiff for web_time::Duration {
    fn as_f32(self) -> f32 {
        self.as_secs_f32()
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, scale: f32) -> Self {
        self.mul_f32(scale)
    }
}
