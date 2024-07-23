use crate::animated::Animated;
use crate::{Easing, TimeDiff};
use crate::{Mix, Time};
use std::fmt::Debug;

/// A value that smoothly goes to the target during a specific time.
/// The target can be changed at any time. No jumps will occur.
/// It's expected that time is always increasing.
/// Every method receives `current_time` as a parameter to allow testing,
/// and have a consistent behavior during a single animation frame.
#[derive(Clone, PartialEq)]
pub struct Inertial<Item: Mix + Clone + PartialEq, X: Time> {
    target: Item,
    start_time: Option<X>,
    duration: X::Duration,
    easing: Easing,
    parent: Option<Box<Inertial<Item, X>>>,
}

impl<Item: Mix + Clone + PartialEq, X: Time> Animated<Item, X> for Inertial<Item, X> {
    fn get(&self, current_time: X) -> Item {
        if let Some(start_time) = self.start_time {
            if current_time < start_time {
                if let Some(parent) = &self.parent {
                    parent.get(current_time)
                } else {
                    self.target.clone()
                }
            } else if self.is_finished(current_time) || self.duration == Default::default() {
                self.target.clone()
            } else if let Some(parent) = &self.parent {
                let elapsed = current_time.since(start_time);

                let t = elapsed.as_f32() / self.duration.as_f32();
                let t = self.easing.ease(t);

                parent.get(current_time).mix(self.target.clone(), t)
            } else {
                self.target.clone()
            }
        } else {
            self.target.clone()
        }
    }

    /// Check if the inertial value reached the target.
    fn is_finished(&self, current_time: X) -> bool {
        self.end_time()
            .map(|end_time| current_time > end_time)
            .unwrap_or(true)
    }
}

impl<Item: Mix + Clone + PartialEq + Debug, X: Time + Debug> Debug for Inertial<Item, X>
where
    X::Duration: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inertial")
            .field("target", &self.target)
            .field("start_time", &self.start_time)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .field("parent", &self.parent)
            .finish()
    }
}

impl<Item: Mix + Clone + PartialEq, X: Time> From<Item> for Inertial<Item, X> {
    fn from(value: Item) -> Self {
        Self::new(value)
    }
}

impl<Item: Mix + Clone + PartialEq + Default, X: Time> Default for Inertial<Item, X>
where
    X::Duration: Default,
{
    fn default() -> Self {
        Self {
            target: Default::default(),
            start_time: Default::default(),
            duration: Default::default(),
            easing: Easing::None,
            parent: None,
        }
    }
}

impl<Item: Mix + Clone + PartialEq, X: Time> Inertial<Item, X> {
    /// Create a new inertial value at a specific time.
    pub fn new(value: Item) -> Self {
        Self {
            target: value,
            start_time: Default::default(),
            duration: Default::default(),
            easing: Easing::None,
            parent: None,
        }
    }

    /// Get the target value.
    pub fn target(&self) -> Item {
        self.target.clone()
    }

    /// Get transition end time.
    pub fn end_time(&self) -> Option<X> {
        self.start_time
            .map(|start_time| start_time.advance(self.duration))
    }

    /// Create child inertial value with a new target at a specific time.
    /// Easing is set to default (`QuadraticInOut`).
    /// * `target` - The new target value.
    /// * `current_time` - The time to start the transition, usually `Instant::now()`.
    /// * `duration` - The duration of the transition.
    pub fn go_to(self, target: Item, current_time: X, duration: X::Duration) -> Self {
        self.ease_to(target, current_time, duration, Easing::default())
    }

    /// Create child inertial value with a new target, easing and start time.
    /// * `target` - The new target value.
    /// * `start_time` - The time to start the transition, usually `Instant::now()`.
    /// * `duration` - The duration of the transition.
    pub fn ease_to(
        self,
        target: Item,
        current_time: X,
        duration: X::Duration,
        easing: Easing,
    ) -> Self {
        if target == self.target {
            self
        } else {
            Self {
                target,
                start_time: Some(current_time),
                duration,
                easing,
                parent: self.clean_up_at(current_time),
            }
        }
    }

    /// Remove all finished ancestors.
    pub(self) fn clean_up_at(self, current_time: X) -> Option<Box<Self>> {
        let is_finished = self.is_finished(current_time);

        Some(Box::new(Self {
            target: self.target,
            start_time: self.start_time,
            duration: self.duration,
            easing: self.easing,
            parent: if is_finished {
                None
            } else {
                self.parent
                    .and_then(|parent| parent.clean_up_at(current_time))
            },
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn new_at() {
        let start_time = Instant::now();
        let inertial = Inertial::new(5);
        assert_eq!(inertial.get(start_time), 5);
        assert_eq!(inertial.get(start_time + Duration::from_secs(1)), 5);
    }

    #[test]
    fn go_to_at() {
        let start_time = Instant::now();
        let inertial = Inertial::new(5.0);

        let new_start_time = start_time + Duration::from_millis(500);
        let new_duration = Duration::from_secs(1);
        let new_inertial = inertial.go_to(10.0, new_start_time, new_duration);

        assert_eq!(new_inertial.get(start_time), 5.0);
        assert_eq!(new_inertial.get(new_start_time), 5.0);
        assert_eq!(
            new_inertial.get(new_start_time + Duration::from_millis(500)),
            7.5
        );
        assert_eq!(new_inertial.get(new_start_time + new_duration), 10.0);
    }
}
