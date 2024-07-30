#![doc = include_str!("../README.md")]

mod animation;
mod easing;
mod inertial;
mod keyframes;
mod mix;
mod stationary;
mod time;

mod animated;
mod impls;
mod smooth_array;

pub use animated::Animated;
pub use animation::Animation;
pub use easing::Easing;
pub use inertial::Inertial;
pub use keyframes::{keyframes, Keyframes};
pub use mix::Mix;
pub use stationary::Stationary;
pub use time::{Time, TimeDiff};

#[cfg(feature = "derive")]
pub use glissade_macro::Mix;

#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    use crate as glissade;
    use crate::Mix;

    #[derive(Mix, PartialEq, Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[test]
    fn test_struct_derive() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 1.0, y: 1.0 };
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(p3, Point { x: 0.5, y: 0.5 });
    }

    #[derive(Mix, PartialEq, Debug)]
    struct Color(f32, f32, f32);

    #[test]
    fn test_tuple_derive() {
        let c1 = Color(0.0, 0.0, 0.0);
        let c2 = Color(1.0, 1.0, 1.0);
        let c3 = c1.mix(c2, 0.5);
        assert_eq!(c3, Color(0.5, 0.5, 0.5));
    }

    #[derive(Mix, PartialEq, Debug)]
    struct Size<T: Mix>
    where
        T: Clone + Copy,
    {
        width: T,
        height: T,
    }

    #[test]
    fn test_generics_derive() {
        let s1 = Size {
            width: 0.0,
            height: 0.0,
        };
        let s2 = Size {
            width: 1.0,
            height: 1.0,
        };
        let s3 = s1.mix(s2, 0.5);
        assert_eq!(
            s3,
            Size {
                width: 0.5,
                height: 0.5,
            }
        );
    }
}
