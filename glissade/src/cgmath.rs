use crate::Mix;
use cgmath::{
    BaseFloat, Deg, Euler, Matrix2, Matrix3, Matrix4, Point1, Point2, Point3, Quaternion, Rad,
    Vector1, Vector2, Vector3, Vector4,
};

impl<S: Mix> Mix for Vector1<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Vector1 {
            x: self.x.mix(other.x, t),
        }
    }
}

impl<S: Mix> Mix for Vector2<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Vector2 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
        }
    }
}

impl<S: Mix> Mix for Vector3<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Vector3 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
            z: self.z.mix(other.z, t),
        }
    }
}

impl<S: Mix> Mix for Vector4<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Vector4 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
            z: self.z.mix(other.z, t),
            w: self.w.mix(other.w, t),
        }
    }
}

impl<S: From<f32> + BaseFloat> Mix for Quaternion<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.slerp(other, t.into())
    }
}

impl<S: Mix> Mix for Deg<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Deg(self.0.mix(other.0, t))
    }
}

impl<S: Mix> Mix for Rad<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Rad(self.0.mix(other.0, t))
    }
}

impl<S: Mix> Mix for Euler<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Euler {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
            z: self.z.mix(other.z, t),
        }
    }
}

impl<S: Mix> Mix for Point1<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Point1 {
            x: self.x.mix(other.x, t),
        }
    }
}

impl<S: Mix> Mix for Point2<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Point2 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
        }
    }
}

impl<S: Mix> Mix for Point3<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Point3 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
            z: self.z.mix(other.z, t),
        }
    }
}

impl<S: Mix> Mix for Matrix2<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Matrix2 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
        }
    }
}

impl<S: Mix> Mix for Matrix3<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Matrix3 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
            z: self.z.mix(other.z, t),
        }
    }
}

impl<S: Mix> Mix for Matrix4<S> {
    fn mix(self, other: Self, t: f32) -> Self {
        Matrix4 {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
            z: self.z.mix(other.z, t),
            w: self.w.mix(other.w, t),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Mix;
    use cgmath::{
        Deg, Euler, Point1, Point2, Point3, Quaternion, Rad, Rotation3, Vector1, Vector2, Vector3,
        Vector4,
    };

    #[test]
    fn test_vector1() {
        let v1 = Vector1 { x: 0.0 };
        let v2 = Vector1 { x: 1.0 };
        let v3 = v1.mix(v2, 0.5);
        assert_eq!(v3, Vector1 { x: 0.5 });
    }

    #[test]
    fn test_vector2() {
        let v1 = Vector2 { x: 0.0, y: 0.0 };
        let v2 = Vector2 { x: 1.0, y: 1.0 };
        let v3 = v1.mix(v2, 0.5);
        assert_eq!(v3, Vector2 { x: 0.5, y: 0.5 });
    }

    #[test]
    fn test_vector3() {
        let v1 = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let v3 = v1.mix(v2, 0.5);
        assert_eq!(
            v3,
            Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5
            }
        );
    }

    #[test]
    fn test_vector4() {
        let v1 = Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        let v2 = Vector4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        let v3 = v1.mix(v2, 0.5);
        assert_eq!(
            v3,
            Vector4 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
                w: 0.5
            }
        );
    }

    #[test]
    fn test_quaternion() {
        let q1 = Quaternion::from_angle_x(Deg(0.0));
        let q2 = Quaternion::from_angle_x(Deg(90.0));
        let q3 = q1.mix(q2, 0.5);
        assert_eq!(q3, Quaternion::from_angle_x(Deg(45.0)));
    }

    #[test]
    fn test_deg() {
        let d1 = Deg(0.0);
        let d2 = Deg(90.0);
        let d3 = d1.mix(d2, 0.5);
        assert_eq!(d3, Deg(45.0));
    }

    #[test]
    fn test_rad() {
        let r1 = Rad(0.0);
        let r2 = Rad(2.0);
        let r3 = r1.mix(r2, 0.5);
        assert_eq!(r3, Rad(1.0));
    }

    #[test]
    fn test_euler() {
        let e1 = Euler::new(Deg(0.0), Deg(0.0), Deg(0.0));
        let e2 = Euler::new(Deg(90.0), Deg(90.0), Deg(90.0));
        let e3 = e1.mix(e2, 0.5);
        assert_eq!(e3, Euler::new(Deg(45.0), Deg(45.0), Deg(45.0)));
    }

    #[test]
    fn test_point1() {
        let p1 = Point1 { x: 0.0 };
        let p2 = Point1 { x: 1.0 };
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(p3, Point1 { x: 0.5 });
    }

    #[test]
    fn test_point2() {
        let p1 = Point2 { x: 0.0, y: 0.0 };
        let p2 = Point2 { x: 1.0, y: 1.0 };
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(p3, Point2 { x: 0.5, y: 0.5 });
    }

    #[test]
    fn test_point3() {
        let p1 = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let p2 = Point3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(
            p3,
            Point3 {
                x: 0.5,
                y: 0.5,
                z: 0.5
            }
        );
    }
}
