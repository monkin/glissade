use crate::{Distance, Mix};
use glam::{
    Affine2, Affine3A, BVec2, BVec3, BVec4, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2,
    DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, Mat2,
    Mat3, Mat3A, Mat4, Quat, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3,
    UVec4, Vec2, Vec3, Vec3A, Vec4,
};
macro_rules! impl_traits_for_vec {
    ($type:ident) => {
        impl Mix for $type {
            fn mix(self, other: Self, t: f32) -> Self {
                self.lerp(other, t.into())
            }
        }

        impl Distance for $type {
            fn distance(self, other: Self) -> f32 {
                $type::distance(self, other) as f32
            }
        }
    };
}

impl_traits_for_vec!(Vec2);
impl_traits_for_vec!(Vec3);
impl_traits_for_vec!(Vec3A);
impl_traits_for_vec!(Vec4);
impl_traits_for_vec!(DVec2);
impl_traits_for_vec!(DVec3);
impl_traits_for_vec!(DVec4);

macro_rules! impl_mix_for_ivec {
    ($type:ident, $to_f:ident, $to_i:ident, $ft:ty) => {
        impl Mix for $type {
            fn mix(self, other: Self, t: f32) -> Self {
                let sf = self.$to_f();
                let of = other.$to_f();
                (sf + (of - sf) * (t as $ft)).$to_i()
            }
        }

        impl Distance for $type {
            fn distance(self, other: Self) -> f32 {
                let sf = self.$to_f();
                let of = other.$to_f();
                (sf - of).length() as f32
            }
        }
    };
}

impl_mix_for_ivec!(I16Vec2, as_vec2, as_i16vec2, f32);
impl_mix_for_ivec!(I16Vec3, as_vec3, as_i16vec3, f32);
impl_mix_for_ivec!(I16Vec4, as_vec4, as_i16vec4, f32);
impl_mix_for_ivec!(U16Vec2, as_vec2, as_u16vec2, f32);
impl_mix_for_ivec!(U16Vec3, as_vec3, as_u16vec3, f32);
impl_mix_for_ivec!(U16Vec4, as_vec4, as_u16vec4, f32);

impl_mix_for_ivec!(IVec2, as_vec2, as_ivec2, f32);
impl_mix_for_ivec!(IVec3, as_vec3, as_ivec3, f32);
impl_mix_for_ivec!(IVec4, as_vec4, as_ivec4, f32);
impl_mix_for_ivec!(UVec2, as_vec2, as_uvec2, f32);
impl_mix_for_ivec!(UVec3, as_vec3, as_uvec3, f32);
impl_mix_for_ivec!(UVec4, as_vec4, as_uvec4, f32);

impl_mix_for_ivec!(I64Vec2, as_dvec2, as_i64vec2, f64);
impl_mix_for_ivec!(I64Vec3, as_dvec3, as_i64vec3, f64);
impl_mix_for_ivec!(I64Vec4, as_dvec4, as_i64vec4, f64);
impl_mix_for_ivec!(U64Vec2, as_dvec2, as_u64vec2, f64);
impl_mix_for_ivec!(U64Vec3, as_dvec3, as_u64vec3, f64);
impl_mix_for_ivec!(U64Vec4, as_dvec4, as_u64vec4, f64);

macro_rules! impl_mix_for_type {
    ($type:ident) => {
        impl Mix for $type {
            fn mix(self, other: Self, t: f32) -> Self {
                self + (other - self) * t
            }
        }
    };
}

impl_mix_for_type!(Mat2);
impl_mix_for_type!(Mat3);
impl_mix_for_type!(Mat3A);
impl_mix_for_type!(Mat4);
impl_mix_for_type!(Quat);

macro_rules! impl_mix_for_dtype {
    ($type:ident) => {
        impl Mix for $type {
            fn mix(self, other: Self, t: f32) -> Self {
                self + (other - self) * (t as f64)
            }
        }
    };
}

impl_mix_for_dtype!(DMat2);
impl_mix_for_dtype!(DMat3);
impl_mix_for_dtype!(DMat4);
impl_mix_for_dtype!(DQuat);

macro_rules! impl_mix_for_bvec {
    ($type:ident) => {
        impl Mix for $type {
            fn mix(self, other: Self, t: f32) -> Self {
                if t <= 0.5 {
                    self
                } else {
                    other
                }
            }
        }
    };
}

impl_mix_for_bvec!(BVec2);
impl_mix_for_bvec!(BVec3);
impl_mix_for_bvec!(BVec4);

impl Mix for Affine2 {
    fn mix(self, other: Self, t: f32) -> Self {
        Self::from_mat2_translation(
            self.matrix2.mix(other.matrix2, t),
            self.translation.mix(other.translation, t),
        )
    }
}

impl Mix for Affine3A {
    fn mix(self, other: Self, t: f32) -> Self {
        Self::from_mat3_translation(
            self.matrix3.mix(other.matrix3, t).into(),
            self.translation.mix(other.translation, t).into(),
        )
    }
}

impl Mix for DAffine2 {
    fn mix(self, other: Self, t: f32) -> Self {
        Self::from_mat2_translation(
            self.matrix2.mix(other.matrix2, t),
            self.translation.mix(other.translation, t),
        )
    }
}

impl Mix for DAffine3 {
    fn mix(self, other: Self, t: f32) -> Self {
        Self::from_mat3_translation(
            self.matrix3.mix(other.matrix3, t),
            self.translation.mix(other.translation, t),
        )
    }
}
