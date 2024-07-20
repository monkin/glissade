use crate::animation_loop::AnimationLoop;
use glissade::{InertialValue, Mix};
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use web_time::{Duration, Instant};
use yew::prelude::*;

#[hook]
pub fn use_inertial<T>(new_value: &T, duration: Duration) -> T
where
    T: Mix + Clone + Debug + PartialEq + 'static,
{
    let now = Instant::now();

    let inertial = use_state_eq({
        let new_value = new_value.clone();
        move || Rc::new(InertialValue::new(new_value))
    });

    let current = use_state_eq(|| inertial.get(now));

    use_effect_with(new_value.clone(), {
        let inertial = inertial.clone();
        move |value: &T| {
            inertial.set(Rc::new(inertial.as_ref().clone().go_to(
                value.clone(),
                now,
                duration,
            )));
        }
    });

    use_effect_with(inertial.deref().clone(), {
        let current = current.clone();
        move |inertial: &Rc<InertialValue<T, Instant>>| {
            let inertial = inertial.clone();
            AnimationLoop::new(move || current.set(inertial.get(Instant::now())))
        }
    });

    current.deref().clone()
}
