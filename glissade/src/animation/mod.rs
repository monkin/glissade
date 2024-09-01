mod animation_struct;
mod keyframes_easing;
mod keyframes_function;
mod keyframes_linear;
mod keyframes_poly;
mod keyframes_repeat;
mod keyframes_repeat_n;
mod keyframes_reverse;
mod keyframes_scale;
mod keyframes_sequential;
mod keyframes_slice;
mod keyframes_stay;
mod keyframes_trait;

pub use animation_struct::Animation;
pub use keyframes_trait::{keyframes, Keyframes};
