use crate::mix::Mix;
use palette::rgb::Rgb;
use palette::{
    Alpha, Hsl, Hsluv, Hsv, Hwb, Lab, LabHue, Lch, Lchuv, Luv, LuvHue, Okhsl, Okhsv, Okhwb, Oklab,
    OklabHue, Oklch, RgbHue, Xyz, Yxy,
};

macro_rules! impl_mix_for_hue {
    ($hue:ident) => {
        impl<T> Mix for $hue<T>
        where
            T: Mix,
        {
            fn mix(self, other: Self, t: f32) -> Self {
                Self::new(self.into_inner().mix(other.into_inner(), t))
            }
        }
    };
}

impl_mix_for_hue!(RgbHue);
impl_mix_for_hue!(LuvHue);
impl_mix_for_hue!(LabHue);
impl_mix_for_hue!(OklabHue);

impl<C, A> Mix for Alpha<C, A>
where
    C: Mix,
    A: Mix,
{
    fn mix(self, other: Self, t: f32) -> Self {
        Alpha {
            color: self.color.mix(other.color, t),
            alpha: self.alpha.mix(other.alpha, t),
        }
    }
}

macro_rules! impl_mix_for_color1 {
    ($color:ident, $c1:ident, $c2:ident, $c3:ident) => {
        impl<C> Mix for $color<C>
        where
            C: Mix,
        {
            fn mix(self, other: Self, t: f32) -> Self {
                Self::new(
                    self.$c1.mix(other.$c1, t),
                    self.$c2.mix(other.$c2, t),
                    self.$c3.mix(other.$c3, t),
                )
            }
        }
    };
}

macro_rules! impl_mix_for_color2 {
    ($color:ident, $c1:ident, $c2:ident, $c3:ident) => {
        impl<S, C> Mix for $color<S, C>
        where
            C: Mix,
        {
            fn mix(self, other: Self, t: f32) -> Self {
                Self::new(
                    self.$c1.mix(other.$c1, t),
                    self.$c2.mix(other.$c2, t),
                    self.$c3.mix(other.$c3, t),
                )
            }
        }
    };
}

impl_mix_for_color2!(Rgb, red, green, blue);
impl_mix_for_color2!(Hsl, hue, saturation, lightness);
impl_mix_for_color2!(Hsv, hue, saturation, value);
impl_mix_for_color2!(Hsluv, hue, saturation, l);
impl_mix_for_color2!(Hwb, hue, whiteness, blackness);
impl_mix_for_color2!(Lab, l, a, b);
impl_mix_for_color2!(Lch, l, chroma, hue);
impl_mix_for_color2!(Lchuv, l, chroma, hue);
impl_mix_for_color2!(Luv, l, u, v);
impl_mix_for_color1!(Okhsl, hue, saturation, lightness);
impl_mix_for_color1!(Okhsv, hue, saturation, value);
impl_mix_for_color1!(Oklab, l, a, b);
impl_mix_for_color1!(Okhwb, hue, whiteness, blackness);
impl_mix_for_color1!(Oklch, l, chroma, hue);
impl_mix_for_color2!(Xyz, x, y, z);
impl_mix_for_color2!(Yxy, x, y, luma);

#[cfg(test)]
mod tests {
    use crate::Mix;
    use palette::rgb::{Rgb, Rgba};
    use palette::{Hsl, Hsv, Lab, Lch, Lchuv, Luv};

    #[test]
    fn mix_rgb() {
        let a: Rgb = Rgb::new(0.0, 0.0, 0.0);
        let b = Rgb::new(1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Rgb::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn mix_rgba() {
        let a: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);
        let b = Rgba::new(1.0, 1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Rgba::new(0.5, 0.5, 0.5, 0.5));
    }

    #[test]
    fn mix_hsl() {
        let a: Hsl = Hsl::new(0.0, 0.0, 0.0);
        let b = Hsl::new(1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Hsl::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn mix_hsv() {
        let a: Hsv = Hsv::new(0.0, 0.0, 0.0);
        let b = Hsv::new(1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Hsv::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn mix_lch() {
        let a: Lch = Lch::new(0.0, 0.0, 0.0);
        let b = Lch::new(1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Lch::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn mix_lchuv() {
        let a: Lchuv = Lchuv::new(0.0, 0.0, 0.0);
        let b = Lchuv::new(1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Lchuv::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn mix_lab() {
        let a: Lab = Lab::new(0.0, 0.0, 0.0);
        let b = Lab::new(1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Lab::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn mix_luv() {
        let a: Luv = Luv::new(0.0, 0.0, 0.0);
        let b = Luv::new(1.0, 1.0, 1.0);
        let c = a.mix(b, 0.5);
        assert_eq!(c, Luv::new(0.5, 0.5, 0.5));
    }
}
