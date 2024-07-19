use crate::Keyframes;
use std::fmt::Debug;
use std::marker::PhantomData;
use web_time::Instant;

/// Running keyframes animation started at a specific time.
#[derive(Clone)]
pub struct Animation<I: Clone + Sized, T: Keyframes<I>> {
    keyframes: T,
    start_time: Instant,
    phantom: PhantomData<I>,
}

impl<I: Clone + Sized, T: Keyframes<I> + Debug> Debug for Animation<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animation")
            .field("keyframes", &self.keyframes)
            .field("start_time", &self.start_time)
            .finish()
    }
}

impl<I: Clone + Sized, T: Keyframes<I>> Animation<I, T> {
    /// Start the animation at a specific time.
    ///
    /// * `keyframes` - The transition to animate.
    /// * `start_time` - The time to start the animation, usually `Instant::now()`.
    pub fn start(keyframes: T, start_time: Instant) -> Self {
        Self {
            keyframes,
            start_time,
            phantom: Default::default(),
        }
    }

    /// Check if the animation is finished at a specific time.
    pub fn is_finished(&self, current_time: Instant) -> bool {
        self.keyframes
            .is_finished(current_time.duration_since(self.start_time))
    }

    /// Get the start time of the animation.
    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    /// Get the end time of the animation.
    /// Infinite animations will panic.
    pub fn end_time(&self) -> Instant {
        self.start_time + self.keyframes.duration()
    }

    /// Get the value of the animation at a specific time.
    /// * `time` - The time to get the value of the animation, usually `Instant::now()`.
    pub fn get(&self, time: Instant) -> I {
        self.keyframes.get(time.duration_since(self.start_time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyframes::LinearKeyframes;
    use web_time::Duration;

    #[test]
    fn it_works() {
        let keyframes = LinearKeyframes::new(0.0, 10.0, Duration::from_secs(1));
        let start_time = Instant::now();
        let animation = Animation::start(keyframes, start_time);
        let result = animation.get(start_time + Duration::from_millis(500));
        assert_eq!(result, 5.0);
    }
}
