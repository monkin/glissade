use crate::animated::Animated;
use crate::{Keyframes, Time};
use std::fmt::Debug;
use std::marker::PhantomData;

/// Running keyframes animation started at a specific time.
#[derive(Clone)]
pub struct Animation<I: Clone + Sized, X: Time, T: Keyframes<I, X>> {
    keyframes: T,
    start_time: X,
    phantom: PhantomData<I>,
}

impl<I: Clone + Sized, X: Time, T: Keyframes<I, X> + Debug> Debug for Animation<I, X, T>
where
    X: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animation")
            .field("keyframes", &self.keyframes)
            .field("start_time", &self.start_time)
            .finish()
    }
}

impl<I: Clone + Sized, X: Time, T: Keyframes<I, X>> Animation<I, X, T> {
    /// Start the animation at a specific time.
    ///
    /// * `keyframes` - The transition to animate.
    /// * `start_time` - The time to start the animation, usually `Instant::now()`.
    pub fn start(keyframes: T, start_time: X) -> Self {
        Self {
            keyframes,
            start_time,
            phantom: Default::default(),
        }
    }

    /// Get the start time of the animation.
    pub fn start_time(&self) -> X {
        self.start_time
    }

    /// Get the end time of the animation.
    /// Infinite animations will panic.
    pub fn end_time(&self) -> X {
        self.start_time.advance(self.keyframes.duration())
    }
}

impl<I: Clone + Sized, X: Time, T: Keyframes<I, X>> Animated<I, X> for Animation<I, X, T> {
    fn get(&self, time: X) -> I {
        self.keyframes.get(time.since(self.start_time))
    }

    fn is_finished(&self, time: X) -> bool {
        self.keyframes.is_finished(time.since(self.start_time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyframes::LinearKeyframes;
    use std::time::Instant;
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
