use crate::Mix;
use euclid::{
    Angle, BoolVector2D, BoolVector3D, Box2D, Box3D, Length, Point2D, Point3D, Rect,
    RigidTransform3D, Rotation2D, Rotation3D, Scale, Size2D, Size3D, Transform2D, Transform3D,
    Translation2D, Translation3D, Vector2D, Vector3D,
};

impl Mix for Angle<f32> {
    fn mix(self, other: Self, factor: f32) -> Self {
        Angle::radians(self.radians.mix(other.radians, factor))
    }
}

impl Mix for Angle<f64> {
    fn mix(self, other: Self, factor: f32) -> Self {
        Angle::radians(self.radians.mix(other.radians, factor))
    }
}

impl Mix for BoolVector2D {
    fn mix(self, other: Self, factor: f32) -> Self {
        if factor <= 0.5 {
            self
        } else {
            other
        }
    }
}

impl Mix for BoolVector3D {
    fn mix(self, other: Self, factor: f32) -> Self {
        if factor <= 0.5 {
            self
        } else {
            other
        }
    }
}

impl<U> Mix for Box2D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Box2D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Box3D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Box3D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Length<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Length<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Point2D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Point2D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Point3D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Point3D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Rect<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Rect<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Size2D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Size2D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Size3D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Size3D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Vector2D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Vector2D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<U> Mix for Vector3D<f32, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }
}

impl<U> Mix for Vector3D<f64, U> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(other, t as f64)
    }
}

impl<S, D> Mix for Translation2D<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Translation2D::from(self.to_vector().mix(other.to_vector(), t))
    }
}

impl<S, D> Mix for Translation2D<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Translation2D::from(self.to_vector().mix(other.to_vector(), t))
    }
}

impl<S, D> Mix for Translation3D<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Translation3D::from(self.to_vector().mix(other.to_vector(), t))
    }
}

impl<S, D> Mix for Translation3D<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Translation3D::from(self.to_vector().mix(other.to_vector(), t))
    }
}

impl<S, D> Mix for Rotation2D<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Rotation2D::radians(self.angle.mix(other.angle, t))
    }
}

impl<S, D> Mix for Rotation2D<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Rotation2D::radians(self.angle.mix(other.angle, t))
    }
}

impl<S, D> Mix for Rotation3D<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, t)
    }
}

impl<S, D> Mix for Rotation3D<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, t as f64)
    }
}

impl<S, D> Mix for Scale<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Self::new(self.0.mix(other.0, t))
    }
}

impl<S, D> Mix for Scale<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Self::new(self.0.mix(other.0, t))
    }
}

impl<S, D> Mix for RigidTransform3D<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        RigidTransform3D::new(
            self.rotation.mix(other.rotation, t),
            self.translation.mix(other.translation, t),
        )
    }
}

impl<S, D> Mix for RigidTransform3D<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        RigidTransform3D::new(
            self.rotation.mix(other.rotation, t),
            self.translation.mix(other.translation, t),
        )
    }
}

impl<S, D> Mix for Transform2D<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Transform2D::new(
            self.m11.mix(other.m11, t),
            self.m12.mix(other.m12, t),
            self.m21.mix(other.m21, t),
            self.m22.mix(other.m22, t),
            self.m31.mix(other.m31, t),
            self.m32.mix(other.m32, t),
        )
    }
}

impl<S, D> Mix for Transform2D<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Transform2D::new(
            self.m11.mix(other.m11, t),
            self.m12.mix(other.m12, t),
            self.m21.mix(other.m21, t),
            self.m22.mix(other.m22, t),
            self.m31.mix(other.m31, t),
            self.m32.mix(other.m32, t),
        )
    }
}

impl<S, D> Mix for Transform3D<f32, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Transform3D::new(
            self.m11.mix(other.m11, t),
            self.m12.mix(other.m12, t),
            self.m13.mix(other.m13, t),
            self.m14.mix(other.m14, t),
            self.m21.mix(other.m21, t),
            self.m22.mix(other.m22, t),
            self.m23.mix(other.m23, t),
            self.m24.mix(other.m24, t),
            self.m31.mix(other.m31, t),
            self.m32.mix(other.m32, t),
            self.m33.mix(other.m33, t),
            self.m34.mix(other.m34, t),
            self.m41.mix(other.m41, t),
            self.m42.mix(other.m42, t),
            self.m43.mix(other.m43, t),
            self.m44.mix(other.m44, t),
        )
    }
}

impl<S, D> Mix for Transform3D<f64, S, D> {
    fn mix(self, other: Self, t: f32) -> Self {
        Transform3D::new(
            self.m11.mix(other.m11, t),
            self.m12.mix(other.m12, t),
            self.m13.mix(other.m13, t),
            self.m14.mix(other.m14, t),
            self.m21.mix(other.m21, t),
            self.m22.mix(other.m22, t),
            self.m23.mix(other.m23, t),
            self.m24.mix(other.m24, t),
            self.m31.mix(other.m31, t),
            self.m32.mix(other.m32, t),
            self.m33.mix(other.m33, t),
            self.m34.mix(other.m34, t),
            self.m41.mix(other.m41, t),
            self.m42.mix(other.m42, t),
            self.m43.mix(other.m43, t),
            self.m44.mix(other.m44, t),
        )
    }
}
