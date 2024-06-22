use nalgebra::{
    ClosedAdd, ClosedMul, ClosedSub, Isometry, Matrix2, Matrix3, Matrix4, Matrix5, Matrix6, Point,
    Quaternion, RealField, Rotation, Scalar, Scale, Translation, Vector1, Vector2, Vector3,
    Vector4, Vector5, Vector6,
};
use num_traits::{One, Zero};

use crate::mix::Mix;

impl<T> Mix for Vector1<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T> Mix for Vector2<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T> Mix for Vector3<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T> Mix for Vector4<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T> Mix for Vector5<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T> Mix for Matrix2<T>
where
    T: Scalar + Mix,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.zip_map(&other, |a, b| a.mix(b, t))
    }
}

impl<T> Mix for Matrix3<T>
where
    T: Scalar + Mix,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.zip_map(&other, |a, b| a.mix(b, t))
    }
}

impl<T> Mix for Matrix4<T>
where
    T: Scalar + Mix,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.zip_map(&other, |a, b| a.mix(b, t))
    }
}

impl<T> Mix for Matrix5<T>
where
    T: Scalar + Mix,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.zip_map(&other, |a, b| a.mix(b, t))
    }
}

impl<T> Mix for Matrix6<T>
where
    T: Scalar + Mix,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.zip_map(&other, |a, b| a.mix(b, t))
    }
}

impl<T> Mix for Vector6<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T, const D: usize> Mix for Point<T, D>
where
    T: Scalar + From<f32> + Zero + One + ClosedAdd + ClosedSub + ClosedMul,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T, const D: usize> Mix for Scale<T, D>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.vector.lerp(&other.vector, T::from(t)).into()
    }
}

impl<T> Mix for Rotation<T, 2>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32> + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.slerp(&other, T::from(t))
    }
}

impl<T> Mix for Rotation<T, 3>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32> + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.slerp(&other, T::from(t))
    }
}

impl<T, const D: usize> Mix for Translation<T, D>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.vector.lerp(&other.vector, T::from(t)).into()
    }
}

impl<T> Mix for Quaternion<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32> + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T> Mix for Isometry<T, Rotation<T, 2>, 2>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32> + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp_slerp(&other, T::from(t))
    }
}

impl<T> Mix for Isometry<T, Rotation<T, 3>, 3>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedSub + ClosedMul + From<f32> + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp_slerp(&other, T::from(t))
    }
}
