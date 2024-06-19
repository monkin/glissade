/// The easing functions are used to provide a smooth transition between two values over time.
/// See: [https://easings.net/](https://easings.net/) for more information.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    /// Default easing function.
    QuarticInOut,
    Step(f32),
    Bezier(f32, f32, f32, f32),
    None,
}

impl Default for Easing {
    fn default() -> Self {
        Easing::QuadraticInOut
    }
}

impl Easing {
    pub fn ease(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Easing::Linear => t,
            Easing::QuadraticIn => t * t,
            Easing::QuadraticOut => t * (2.0 - t),
            Easing::QuadraticInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    let t = -2.0 * t + 2.0;
                    1.0 - t * t * 0.5
                }
            }
            Easing::CubicIn => t * t * t,
            Easing::CubicOut => {
                let t = 1.0 - t;
                1.0 - t * t * t
            }
            Easing::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t = -2.0 * t + 2.0;
                    1.0 - t * t * t / 2.0
                }
            }
            Easing::QuarticIn => t * t * t * t,
            Easing::QuarticOut => {
                let t = t - 1.0;
                let t = t * t;
                1.0 - t * t
            }
            Easing::QuarticInOut => {
                if t < 0.5 {
                    let t = t * t;
                    8.0 * t * t
                } else {
                    let t = -2.0 * t + 2.0;
                    let t = t * t;
                    1.0 - t * t / 2.0
                }
            }
            Easing::Bezier(p0, p1, p2, p3) => {
                let nt = 1.0 - t;
                let t2 = t * t;
                let nt2 = nt * nt;
                nt * nt2 * p0 + 3.0 * t * nt2 * p1 + 3.0 * t2 * nt * p2 + t2 * t * p3
            }
            Easing::Step(steps) => (t * steps).floor() / steps,
            Easing::None => 1.0,
        }
    }
}
