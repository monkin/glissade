use crate::transition_item::TransitionItem;
use crate::Easing;
use web_time::{Duration, SystemTime};

/// A value that smoothly goes to the target during a specific time.
/// The target can be changed at any time. No jumps will occur.
/// It's expected that time is always increasing.
/// Every method receives `current_time` as a parameter to allow testing,
/// and have a consistent behavior during a single animation frame.
#[derive(Clone, Debug, PartialEq)]
pub struct InertialValue<T: TransitionItem> {
    target: T,
    start_time: SystemTime,
    duration: Duration,
    easing: Easing,
    parent: Option<Box<InertialValue<T>>>,
}

impl<T: TransitionItem> InertialValue<T> {
    /// Create a new inertial value at a specific time.
    pub fn new(value: T) -> Self {
        Self {
            target: value,
            start_time: SystemTime::UNIX_EPOCH,
            duration: Duration::default(),
            easing: Easing::None,
            parent: None,
        }
    }

    /// Check if the inertial value reached the target.
    pub fn is_finished(&self, current_time: SystemTime) -> bool {
        current_time > self.start_time + self.duration
    }

    /// Get the target value.
    pub fn target(&self) -> T {
        self.target.clone()
    }

    /// Get transition end time.
    pub fn end_time(&self) -> SystemTime {
        self.start_time + self.duration
    }

    /// Get the value of the inertial value at a specific time.
    /// * `current_time` - The time to get the value of the inertial value, usually `SystemTime::now()`.
    pub fn get(&self, current_time: SystemTime) -> T {
        if current_time < self.start_time {
            if let Some(parent) = &self.parent {
                parent.get(current_time)
            } else {
                self.target.clone()
            }
        } else if self.is_finished(current_time) {
            self.target.clone()
        } else if let Some(parent) = &self.parent {
            let elapsed = current_time
                .duration_since(self.start_time)
                .unwrap_or_default();

            let t = elapsed.as_secs_f32() / self.duration.as_secs_f32();
            let t = self.easing.ease(t);

            parent.get(current_time).mix(self.target.clone(), t)
        } else {
            self.target.clone()
        }
    }

    /// Create child inertial value with a new target at a specific time.
    /// Easing is set to default (`QuadraticInOut`).
    /// * `target` - The new target value.
    /// * `current_time` - The time to start the transition, usually `SystemTime::now()`.
    /// * `duration` - The duration of the transition.
    pub fn go_to(self, target: T, current_time: SystemTime, duration: Duration) -> Self {
        self.ease_to(target, current_time, duration, Easing::default())
    }

    /// Create child inertial value with a new target, easing and start time.
    /// * `target` - The new target value.
    /// * `start_time` - The time to start the transition, usually `SystemTime::now()`.
    /// * `duration` - The duration of the transition.
    pub fn ease_to(
        self,
        target: T,
        current_time: SystemTime,
        duration: Duration,
        easing: Easing,
    ) -> Self {
        if target == self.target {
            self
        } else {
            Self {
                target,
                start_time: current_time,
                duration,
                easing,
                parent: self.clean_up_at(current_time),
            }
        }
    }

    /// Remove all finished ancestors.
    pub(self) fn clean_up_at(self, current_time: SystemTime) -> Option<Box<Self>> {
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

    #[test]
    fn new_at() {
        let start_time = SystemTime::now();
        let inertial_value = InertialValue::new(5);
        assert_eq!(inertial_value.get(start_time), 5);
        assert_eq!(inertial_value.get(start_time + Duration::from_secs(1)), 5);
    }

    #[test]
    fn go_to_at() {
        let start_time = SystemTime::now();
        let inertial_value = InertialValue::new(5.0);

        let new_start_time = start_time + Duration::from_millis(500);
        let new_duration = Duration::from_secs(1);
        let new_inertial_value = inertial_value.go_to(10.0, new_start_time, new_duration);

        assert_eq!(new_inertial_value.get(start_time), 5.0);
        assert_eq!(new_inertial_value.get(new_start_time), 5.0);
        assert_eq!(
            new_inertial_value.get(new_start_time + Duration::from_millis(500)),
            7.5
        );
        assert_eq!(new_inertial_value.get(new_start_time + new_duration), 10.0);
    }
}
