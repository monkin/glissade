use nalgebra::{
    ClosedAddAssign, ClosedMulAssign, ClosedSubAssign, Isometry, Matrix1x2, Matrix1x3, Matrix1x4,
    Matrix1x5, Matrix1x6, Matrix2, Matrix2x3, Matrix2x4, Matrix2x5, Matrix2x6, Matrix3, Matrix3x2,
    Matrix3x4, Matrix3x5, Matrix3x6, Matrix4, Matrix4x2, Matrix4x3, Matrix4x5, Matrix4x6, Matrix5,
    Matrix5x2, Matrix5x3, Matrix5x4, Matrix5x6, Matrix6, Matrix6x2, Matrix6x3, Matrix6x4,
    Matrix6x5, Point, Quaternion, RealField, Rotation, Scalar, Scale, Translation, Vector1,
    Vector2, Vector3, Vector4, Vector5, Vector6,
};
use num_traits::{One, Zero};

use crate::mix::Mix;
use crate::Stationary;

macro_rules! impl_traits_for_vector {
    ($vector:ident) => {
        impl<T> Mix for $vector<T>
        where
            T: Scalar
                + Zero
                + One
                + ClosedAddAssign
                + ClosedSubAssign
                + ClosedMulAssign
                + From<f32>,
        {
            fn mix(self, other: Self, t: f32) -> Self {
                self.lerp(&other, T::from(t))
            }
        }

        impl<T: Clone> Stationary for $vector<T> {}
    };
}

impl_traits_for_vector!(Vector1);
impl_traits_for_vector!(Vector2);
impl_traits_for_vector!(Vector3);
impl_traits_for_vector!(Vector4);
impl_traits_for_vector!(Vector5);
impl_traits_for_vector!(Vector6);

macro_rules! impl_mix_for_matrix {
    ($matrix:ident) => {
        impl<T> Mix for $matrix<T>
        where
            T: Scalar + Mix,
        {
            fn mix(self, other: Self, t: f32) -> Self {
                self.zip_map(&other, |a, b| a.mix(b, t))
            }
        }

        impl<T: Clone> Stationary for $matrix<T> {}
    };
}

impl_mix_for_matrix!(Matrix1x2);
impl_mix_for_matrix!(Matrix2);
impl_mix_for_matrix!(Matrix3x2);
impl_mix_for_matrix!(Matrix4x2);
impl_mix_for_matrix!(Matrix5x2);
impl_mix_for_matrix!(Matrix6x2);

impl_mix_for_matrix!(Matrix1x3);
impl_mix_for_matrix!(Matrix2x3);
impl_mix_for_matrix!(Matrix3);
impl_mix_for_matrix!(Matrix4x3);
impl_mix_for_matrix!(Matrix5x3);
impl_mix_for_matrix!(Matrix6x3);

impl_mix_for_matrix!(Matrix1x4);
impl_mix_for_matrix!(Matrix2x4);
impl_mix_for_matrix!(Matrix3x4);
impl_mix_for_matrix!(Matrix4);
impl_mix_for_matrix!(Matrix5x4);
impl_mix_for_matrix!(Matrix6x4);

impl_mix_for_matrix!(Matrix1x5);
impl_mix_for_matrix!(Matrix2x5);
impl_mix_for_matrix!(Matrix3x5);
impl_mix_for_matrix!(Matrix4x5);
impl_mix_for_matrix!(Matrix5);
impl_mix_for_matrix!(Matrix6x5);

impl_mix_for_matrix!(Matrix1x6);
impl_mix_for_matrix!(Matrix2x6);
impl_mix_for_matrix!(Matrix3x6);
impl_mix_for_matrix!(Matrix4x6);
impl_mix_for_matrix!(Matrix5x6);
impl_mix_for_matrix!(Matrix6);

impl<T, const D: usize> Mix for Point<T, D>
where
    T: Scalar + From<f32> + Zero + One + ClosedAddAssign + ClosedSubAssign + ClosedMulAssign,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T, const D: usize> Stationary for Point<T, D> where T: Clone {}

impl<T, const D: usize> Mix for Scale<T, D>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedSubAssign + ClosedMulAssign + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.vector.lerp(&other.vector, T::from(t)).into()
    }
}

impl<T, const D: usize> Stationary for Scale<T, D> where T: Clone {}

impl<T> Mix for Rotation<T, 2>
where
    T: Scalar
        + Zero
        + One
        + ClosedAddAssign
        + ClosedSubAssign
        + ClosedMulAssign
        + From<f32>
        + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.slerp(&other, T::from(t))
    }
}

impl<T: Clone> Stationary for Rotation<T, 2> {}

impl<T> Mix for Rotation<T, 3>
where
    T: Scalar
        + Zero
        + One
        + ClosedAddAssign
        + ClosedSubAssign
        + ClosedMulAssign
        + From<f32>
        + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.slerp(&other, T::from(t))
    }
}

impl<T: Clone> Stationary for Rotation<T, 3> {}

impl<T, const D: usize> Mix for Translation<T, D>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedSubAssign + ClosedMulAssign + From<f32>,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.vector.lerp(&other.vector, T::from(t)).into()
    }
}

impl<T: Clone, const D: usize> Stationary for Translation<T, D> {}

impl<T> Mix for Quaternion<T>
where
    T: Scalar
        + Zero
        + One
        + ClosedAddAssign
        + ClosedSubAssign
        + ClosedMulAssign
        + From<f32>
        + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp(&other, T::from(t))
    }
}

impl<T> Stationary for Quaternion<T> where T: Clone {}

impl<T> Mix for Isometry<T, Rotation<T, 2>, 2>
where
    T: Scalar
        + Zero
        + One
        + ClosedAddAssign
        + ClosedSubAssign
        + ClosedMulAssign
        + From<f32>
        + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp_slerp(&other, T::from(t))
    }
}

impl<T> Stationary for Isometry<T, Rotation<T, 2>, 2> where T: Clone {}

impl<T> Mix for Isometry<T, Rotation<T, 3>, 3>
where
    T: Scalar
        + Zero
        + One
        + ClosedAddAssign
        + ClosedSubAssign
        + ClosedMulAssign
        + From<f32>
        + RealField,
{
    fn mix(self, other: Self, t: f32) -> Self {
        self.lerp_slerp(&other, T::from(t))
    }
}

impl<T> Stationary for Isometry<T, Rotation<T, 3>, 3> where T: Clone {}

#[cfg(test)]
mod tests {
    use nalgebra::{
        Point2, Point3, Quaternion, Rotation2, Translation2, Translation3, Vector2, Vector3,
        Vector4,
    };

    use crate::Mix;

    #[test]
    fn test_point2_mix() {
        let p1 = Point2::new(1.0, 2.0);
        let p2 = Point2::new(3.0, 4.0);
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(p3, Point2::new(2.0, 3.0));
    }

    #[test]
    fn test_point3_mix() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = Point3::new(4.0, 5.0, 6.0);
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(p3, Point3::new(2.5, 3.5, 4.5));
    }

    #[test]
    fn test_translation2_mix() {
        let t1 = Translation2::new(1.0, 2.0);
        let t2 = Translation2::new(3.0, 4.0);
        let t3 = t1.mix(t2, 0.5);
        assert_eq!(t3, Translation2::new(2.0, 3.0));
    }

    #[test]
    fn test_translation3_mix() {
        let t1 = Translation3::new(1.0, 2.0, 3.0);
        let t2 = Translation3::new(4.0, 5.0, 6.0);
        let t3 = t1.mix(t2, 0.5);
        assert_eq!(t3, Translation3::new(2.5, 3.5, 4.5));
    }

    #[test]
    fn test_rotation2_mix() {
        let r1 = Rotation2::new(0.5);
        let r2 = Rotation2::new(1.0);
        let r3 = r1.mix(r2, 0.5);
        assert_eq!(r3.angle(), 0.75);
    }

    #[test]
    fn test_quaternion_mix() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
        let q3 = q1.mix(q2, 0.5);
        assert_eq!(q3, Quaternion::new(3.0, 4.0, 5.0, 6.0));
    }

    #[test]
    fn test_point2_mix_f32() {
        let p1 = Point2::new(1.0f32, 2.0f32);
        let p2 = Point2::new(3.0f32, 4.0f32);
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(p3, Point2::new(2.0, 3.0));
    }

    #[test]
    fn test_point3_mix_f32() {
        let p1 = Point3::new(1.0f32, 2.0f32, 3.0f32);
        let p2 = Point3::new(4.0f32, 5.0f32, 6.0f32);
        let p3 = p1.mix(p2, 0.5);
        assert_eq!(p3, Point3::new(2.5, 3.5, 4.5));
    }

    #[test]
    fn test_translation2_mix_f32() {
        let t1 = Translation2::new(1.0f32, 2.0f32);
        let t2 = Translation2::new(3.0f32, 4.0f32);
        let t3 = t1.mix(t2, 0.5);
        assert_eq!(t3, Translation2::new(2.0, 3.0));
    }

    #[test]
    fn test_translation3_mix_f32() {
        let t1 = Translation3::new(1.0f32, 2.0f32, 3.0f32);
        let t2 = Translation3::new(4.0f32, 5.0f32, 6.0f32);
        let t3 = t1.mix(t2, 0.5);
        assert_eq!(t3, Translation3::new(2.5, 3.5, 4.5));
    }

    #[test]
    fn test_vector2_mix() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        let v3 = v1.mix(v2, 0.5);
        assert_eq!(v3, Vector2::new(2.0, 3.0));
    }

    #[test]
    fn test_vector3_mix() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1.mix(v2, 0.5);
        assert_eq!(v3, Vector3::new(2.5, 3.5, 4.5));
    }

    #[test]
    fn test_vector4_mix() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let v3 = v1.mix(v2, 0.5);
        assert_eq!(v3, Vector4::new(3.0, 4.0, 5.0, 6.0));
    }
}
