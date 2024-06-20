/// The easing functions are used to provide a smooth transition between two values over time.
/// See: [https://easings.net/](https://easings.net/) for more information.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
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
    Bezier(f32, f32, f32, f32),
    /// <div>
    ///     <img style="width: 102px; height: 102px;" src="data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjEwMiIgd2lkdGg9IjEwMiIgdmlld0JveD0iLTEgLTEgMTAyIDEwMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB4PSItMSIgeT0iLTEiIHdpZHRoPSIxMDIiIGhlaWdodD0iMTAyIiBmaWxsPSJyZ2JhKDAsIDAsIDAsIDAuMTIpIi8+CiAgPHBvbHlsaW5lIHBvaW50cz0iMCwwIDEwMCwwIiBzdHlsZT0ic3Ryb2tlOiBibGFjazsgc3Ryb2tlLXdpZHRoOiAxOyBmaWxsOiBub25lOyIgLz4KPC9zdmc+"/>
    /// </div>
    None,
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
