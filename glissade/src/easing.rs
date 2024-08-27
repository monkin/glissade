use crate::smooth_array::SmoothArray;

const BEZIER_POINTS_COUNT: usize = 128;

/// The easing functions are used to provide a smooth transition between two values over time.
/// See: [https://easings.net/](https://easings.net/) for more information.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Easing {
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMCIgd2lkdGg9IjEwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCIgZmlsbD0icmdiYSgwLCAwLCAwLCAwLjEyKSIvPgogIDxwb2x5Z29uIHBvaW50cz0iMCwgMTAwIDEwMCwgMCIgc3R5bGU9InN0cm9rZTogYmxhY2s7IHN0cm9rZS13aWR0aDogMTsgZmlsbDogbm9uZTsiIC8+Cjwvc3ZnPg=="/>
    /// </div>
    Linear,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSw5OS43NSAxMCw5OSAxNSw5Ny43NSAyMCw5NiAyNSw5My43NSAzMCw5MSAzNSw4Ny43NSA0MCw4NCA0NSw3OS43NSA1MCw3NSA1NSw2OS43NSA2MCw2NCA2NSw1Ny43NSA3MCw1MSA3NSw0My43NSA4MCwzNiA4NSwyNy43NSA5MCwxOSA5NSw5Ljc1IDEwMCwwIiBzdHlsZT0ic3Ryb2tlOiBibGFjazsgc3Ryb2tlLXdpZHRoOiAxOyBmaWxsOiBub25lOyIgLz4KPC9zdmc+"/>
    /// </div>
    QuadraticIn,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSw5MC4yNSAxMCw4MSAxNSw3Mi4yNSAyMCw2NCAyNSw1Ni4yNSAzMCw0OSAzNSw0Mi4yNSA0MCwzNiA0NSwzMC4yNSA1MCwyNSA1NSwyMC4yNSA2MCwxNiA2NSwxMi4yNSA3MCw5IDc1LDYuMjUgODAsNCA4NSwyLjI1IDkwLDEgOTUsMC4yNSAxMDAsMCIgc3R5bGU9InN0cm9rZTogYmxhY2s7IHN0cm9rZS13aWR0aDogMTsgZmlsbDogbm9uZTsiIC8+Cjwvc3ZnPg=="/>
    /// </div>
    QuadraticOut,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSw5OS41IDEwLDk4IDE1LDk1LjUgMjAsOTIgMjUsODcuNSAzMCw4MiAzNSw3NS41IDQwLDY4IDQ1LDU5LjUgNTAsNTAgNTUsNDAuNSA2MCwzMiA2NSwyNC41IDcwLDE4IDc1LDEyLjUgODAsOCA4NSw0LjUgOTAsMiA5NSwwLjUgMTAwLDAiIHN0eWxlPSJzdHJva2U6IGJsYWNrOyBzdHJva2Utd2lkdGg6IDE7IGZpbGw6IG5vbmU7IiAvPgo8L3N2Zz4="/>
    /// </div>
    #[default]
    QuadraticInOut,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSwxMDAgMTAsOTkuODc1IDE1LDk5LjYyNSAyMCw5OS4yNSAyNSw5OC41IDMwLDk3LjI1IDM1LDk1Ljc1IDQwLDkzLjYyNSA0NSw5MC44NzUgNTAsODcuNSA1NSw4My4zNzUgNjAsNzguMzc1IDY1LDcyLjUgNzAsNjUuNzUgNzUsNTcuODc1IDgwLDQ4Ljc1IDg1LDM4LjYyNSA5MCwyNy4xMjUgOTUsMTQuMjUgMTAwLDAiIHN0eWxlPSJzdHJva2U6IGJsYWNrOyBzdHJva2Utd2lkdGg6IDE7IGZpbGw6IG5vbmU7IiAvPgo8L3N2Zz4="/>
    /// </div>
    CubicIn,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSw4NS43NSAxMCw3Mi44NzUgMTUsNjEuMzc1IDIwLDUxLjI1IDI1LDQyLjI1IDMwLDM0LjI1IDM1LDI3LjUgNDAsMjEuNjI1IDQ1LDE2LjYyNSA1MCwxMi41IDU1LDkuMTI1IDYwLDYuMzc1IDY1LDQuMjUgNzAsMi43NSA3NSwxLjYyNSA4MCwwLjc1IDg1LDAuMzc1IDkwLDAuMTI1IDk1LDAgMTAwLDAiIHN0eWxlPSJzdHJva2U6IGJsYWNrOyBzdHJva2Utd2lkdGg6IDE7IGZpbGw6IG5vbmU7IiAvPgo8L3N2Zz4="/>
    /// </div>
    CubicOut,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSwxMDAgMTAsOTkuNjI1IDE1LDk4LjYyNSAyMCw5Ni43NSAyNSw5My43NSAzMCw4OS4yNSAzNSw4Mi44NzUgNDAsNzQuMzc1IDQ1LDYzLjUgNTAsNTAgNTUsMzYuNSA2MCwyNS42MjUgNjUsMTcuMTI1IDcwLDEwLjc1IDc1LDYuMjUgODAsMy4yNSA4NSwxLjM3NSA5MCwwLjM3NSA5NSwwIDEwMCwwIiBzdHlsZT0ic3Ryb2tlOiBibGFjazsgc3Ryb2tlLXdpZHRoOiAxOyBmaWxsOiBub25lOyIgLz4KPC9zdmc+"/>
    /// </div>
    CubicInOut,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSwxMDAgMTAsMTAwIDE1LDEwMCAyMCw5OS44NzUgMjUsOTkuNjI1IDMwLDk5LjI1IDM1LDk4LjUgNDAsOTcuNSA0NSw5NS44NzUgNTAsOTMuNzUgNTUsOTAuODc1IDYwLDg3IDY1LDgyLjEyNSA3MCw3NiA3NSw2OC4zNzUgODAsNTkgODUsNDcuNzUgOTAsMzQuMzc1IDk1LDE4LjUgMTAwLDAiIHN0eWxlPSJzdHJva2U6IGJsYWNrOyBzdHJva2Utd2lkdGg6IDE7IGZpbGw6IG5vbmU7IiAvPgo8L3N2Zz4="/>
    /// </div>
    QuarticIn,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSw4MS41IDEwLDY1LjYyNSAxNSw1Mi4yNSAyMCw0MSAyNSwzMS42MjUgMzAsMjQgMzUsMTcuODc1IDQwLDEzIDQ1LDkuMTI1IDUwLDYuMjUgNTUsNC4xMjUgNjAsMi41IDY1LDEuNSA3MCwwLjc1IDc1LDAuMzc1IDgwLDAuMTI1IDg1LDAgOTAsMCA5NSwwIDEwMCwwIiBzdHlsZT0ic3Ryb2tlOiBibGFjazsgc3Ryb2tlLXdpZHRoOiAxOyBmaWxsOiBub25lOyIgLz4KPC9zdmc+"/>
    /// </div>
    QuarticOut,
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwxMDAgNSwxMDAgMTAsOTkuODc1IDE1LDk5LjYyNSAyMCw5OC43NSAyNSw5Ni44NzUgMzAsOTMuNSAzNSw4OCA0MCw3OS41IDQ1LDY3LjI1IDUwLDUwIDU1LDMyLjc1IDYwLDIwLjUgNjUsMTIgNzAsNi41IDc1LDMuMTI1IDgwLDEuMjUgODUsMC4zNzUgOTAsMC4xMjUgOTUsMCAxMDAsMCIgc3R5bGU9InN0cm9rZTogYmxhY2s7IHN0cm9rZS13aWR0aDogMTsgZmlsbDogbm9uZTsiIC8+Cjwvc3ZnPg=="/>
    /// </div>
    QuarticInOut,

    /// Step(4)
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiB2aWV3Qm94PSItMSAtMSAxMDIgMTAyIj4KPHBhdGggZmlsbD0icmdiYSgwLCAwLCAwLCAwLjEyKSIgZD0iTS0xLTFoMTAydjEwMkgtMXoiLz48cGF0aCBkPSJNMCAxMDBoMjVsMC0yNWgyNWwwLTI1aDI1bDAtMjVoMjVsMC0yNSIgc3R5bGU9InN0cm9rZTojMDAwO3N0cm9rZS13aWR0aDoxO2ZpbGw6bm9uZSIvPjwvc3ZnPg=="/>
    /// </div>
    ///
    /// Step(10)
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiB2aWV3Qm94PSItMSAtMSAxMDIgMTAyIj48cGF0aCBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIiBkPSJNLTEtMWgxMDJ2MTAySC0xeiIvPjxwYXRoIGQ9Ik0wIDEwMGgxMFY5MGgxMFY4MGgxMFY3MGgxMFY2MGgxMFY1MGgxMFY0MGgxMFYzMGgxMFYyMGgxMFYxMGgxMFYwIiBzdHlsZT0ic3Ryb2tlOiMwMDA7c3Ryb2tlLXdpZHRoOjE7ZmlsbDpub25lIi8+PC9zdmc+"/>
    /// </div>
    Step(f32),

    /// Easing described by a table of values. Values in between are interpolated.
    /// For example, `Easing::Tabular(vec![0.0, 0.1, 0.2, 0.4, 0.8, 1.0].into())`
    Tabular(SmoothArray),

    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwwIDEwMCwwIiBzdHlsZT0ic3Ryb2tlOiBibGFjazsgc3Ryb2tlLXdpZHRoOiAxOyBmaWxsOiBub25lOyIgLz4KPC9zdmc+"/>
    /// </div>
    None,
}

impl Easing {
    pub fn ease(&self, t: f32) -> f32 {
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
            Easing::Tabular(data) => data.value_at(t),
            Easing::Step(steps) => (t * steps).floor() / steps,
            Easing::None => 1.0,
        }
    }

    /// For more information see: [https://cubic-bezier.com/](https://cubic-bezier.com/)
    ///
    /// Bezier(0.17, 0.67, 0.7, 0.05)
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22102%22%20height%3D%22102%22%20viewBox%3D%22-1%20-1%20102%20102%22%3E%3Cpath%20fill%3D%22rgba(0%2C%200%2C%200%2C%200.12)%22%20d%3D%22M-1-1h102v102H-1z%22%2F%3E%3Cpath%20d%3D%22M0%20100%20C17%2C33%2C70%2C95%2C100%2C0%22%20style%3D%22stroke%3A%23000%3Bstroke-width%3A1%3Bfill%3Anone%22%2F%3E%3C%2Fsvg%3E"/>
    /// </div>
    ///
    /// Bezier(0.98, 0.62, 0.42, 0.93)
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22102%22%20height%3D%22102%22%20viewBox%3D%22-1%20-1%20102%20102%22%3E%3Cpath%20fill%3D%22rgba(0%2C%200%2C%200%2C%200.12)%22%20d%3D%22M-1-1h102v102H-1z%22%2F%3E%3Cpath%20d%3D%22M0%20100%20C98%2C38%2C42%2C7%2C100%2C0%22%20style%3D%22stroke%3A%23000%3Bstroke-width%3A1%3Bfill%3Anone%22%2F%3E%3C%2Fsvg%3E"/>
    /// </div>
    pub fn bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> Easing {
        let x1 = x1.clamp(0.0, 1.0);
        let x2 = x2.clamp(0.0, 1.0);

        let mut data = SmoothArray::new(BEZIER_POINTS_COUNT);

        let mut previous = (0.0, 0.0);
        for i in 1..=BEZIER_POINTS_COUNT {
            let t = i as f32 / BEZIER_POINTS_COUNT as f32;
            let nt = 1.0 - t;
            let t2 = t * t;
            let nt2 = nt * nt;

            let x = (3.0 * nt2 * t * x1 + 3.0 * nt * t2 * x2 + t2 * t).clamp(0.0, 1.0);
            let y = 3.0 * nt2 * t * y1 + 3.0 * nt * t2 * y2 + t2 * t;

            data.line(previous, (x, y));
            previous = (x, y);
        }

        Easing::Tabular(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear() {
        let easing = Easing::Linear;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.5);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn quadratic_in() {
        let easing = Easing::QuadraticIn;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.25);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn quadratic_out() {
        let easing = Easing::QuadraticOut;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.75);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn quadratic_in_out() {
        let easing = Easing::QuadraticInOut;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.5);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn cubic_in() {
        let easing = Easing::CubicIn;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.125);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn cubic_out() {
        let easing = Easing::CubicOut;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.875);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn cubic_in_out() {
        let easing = Easing::CubicInOut;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.5);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn bezier() {
        let easing = Easing::bezier(0.0, 0.0, 1.0, 1.0);
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.5);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn bezier_non_linear() {
        let easing = Easing::bezier(0.0, 0.5, 1.0, 0.5);
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.5);
        assert_eq!(easing.ease(1.0), 1.0);
    }
}
