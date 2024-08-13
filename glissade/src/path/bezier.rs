use crate::path::curve::Curve;
use crate::{Distance, Mix};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Bezier0<T: Distance + Mix + Clone>(pub T);
#[derive(Clone)]
pub struct Bezier1<T: Distance + Mix + Clone>(pub T, pub T);
#[derive(Clone)]
pub struct Bezier2<T: Distance + Mix + Clone>(pub T, pub T, pub T);
#[derive(Clone)]
pub struct Bezier3<T: Distance + Mix + Clone>(pub T, pub T, pub T, pub T);

impl<T: Distance + Mix + Clone> Bezier0<T> {
    pub fn new(p: T) -> Self {
        Self(p)
    }
}

impl<T: Distance + Mix + Clone> Bezier1<T> {
    pub fn new(p0: T, p1: T) -> Self {
        Self(p0, p1)
    }
}

impl<T: Distance + Mix + Clone> Bezier2<T> {
    pub fn new(p0: T, p1: T, p2: T) -> Self {
        Self(p0, p1, p2)
    }
}

impl<T: Distance + Mix + Clone> Bezier3<T> {
    pub fn new(p0: T, p1: T, p2: T, p3: T) -> Self {
        Self(p0, p1, p2, p3)
    }
}

impl<T: Distance + Mix + Clone + Copy> Copy for Bezier0<T> {}
impl<T: Distance + Mix + Clone + Copy> Copy for Bezier1<T> {}
impl<T: Distance + Mix + Clone + Copy> Copy for Bezier2<T> {}
impl<T: Distance + Mix + Clone + Copy> Copy for Bezier3<T> {}

impl<T: Distance + Mix + Clone + Debug> Debug for Bezier0<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier0").field(&self.0).finish()
    }
}

impl<T: Distance + Mix + Clone + Debug> Debug for Bezier1<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier1")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<T: Distance + Mix + Clone + Debug> Debug for Bezier2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier2")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl<T: Distance + Mix + Clone + Debug> Debug for Bezier3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier3")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .field(&self.3)
            .finish()
    }
}

impl<T: Distance + Mix + Clone> Curve<T> for Bezier0<T> {
    fn value_at(&self, _t: f32) -> T {
        self.0.clone()
    }

    fn estimate_length(&self) -> f32 {
        0.0
    }

    fn first_point(&self) -> T {
        self.0.clone()
    }

    fn last_point(&self) -> T {
        self.0.clone()
    }
}

impl<T: Distance + Mix + Clone> Curve<T> for Bezier1<T> {
    fn value_at(&self, t: f32) -> T {
        self.0.clone().mix(self.1.clone(), t)
    }

    fn estimate_length(&self) -> f32 {
        self.0.clone().distance(self.1.clone())
    }

    fn first_point(&self) -> T {
        self.0.clone()
    }

    fn last_point(&self) -> T {
        self.1.clone()
    }
}

impl<T: Distance + Mix + Clone> Curve<T> for Bezier2<T> {
    fn value_at(&self, t: f32) -> T {
        let v01 = self.0.clone().mix(self.1.clone(), t);
        let v12 = self.1.clone().mix(self.2.clone(), t);
        v01.mix(v12, t)
    }

    fn estimate_length(&self) -> f32 {
        let l_max =
            self.0.clone().distance(self.1.clone()) + self.1.clone().distance(self.2.clone());
        let l_min = self.0.clone().distance(self.2.clone());

        (l_max + l_min) * 0.5
    }

    fn first_point(&self) -> T {
        self.0.clone()
    }

    fn last_point(&self) -> T {
        self.2.clone()
    }
}

impl<T: Distance + Mix + Clone> Curve<T> for Bezier3<T> {
    fn value_at(&self, t: f32) -> T {
        let v01 = self.0.clone().mix(self.1.clone(), t);
        let v12 = self.1.clone().mix(self.2.clone(), t);
        let v23 = self.2.clone().mix(self.3.clone(), t);

        let v012 = v01.clone().mix(v12.clone(), t);
        let v123 = v12.clone().mix(v23.clone(), t);

        v012.mix(v123, t)
    }

    fn estimate_length(&self) -> f32 {
        let l_max = self.0.clone().distance(self.1.clone())
            + self.1.clone().distance(self.2.clone())
            + self.2.clone().distance(self.3.clone());
        let l_min = self.0.clone().distance(self.3.clone());

        (l_max + l_min) * 0.5
    }

    fn first_point(&self) -> T {
        self.0.clone()
    }

    fn last_point(&self) -> T {
        self.3.clone()
    }
}

#[derive(Clone)]
pub enum Bezier<T: Distance + Mix + Clone> {
    Bezier0(Bezier0<T>),
    Bezier1(Bezier1<T>),
    Bezier2(Bezier2<T>),
    Bezier3(Bezier3<T>),
}

impl<T: Distance + Mix + Clone> Curve<T> for Bezier<T> {
    fn value_at(&self, t: f32) -> T {
        match self {
            Bezier::Bezier0(b) => b.value_at(t),
            Bezier::Bezier1(b) => b.value_at(t),
            Bezier::Bezier2(b) => b.value_at(t),
            Bezier::Bezier3(b) => b.value_at(t),
        }
    }

    fn estimate_length(&self) -> f32 {
        match self {
            Bezier::Bezier0(b) => b.estimate_length(),
            Bezier::Bezier1(b) => b.estimate_length(),
            Bezier::Bezier2(b) => b.estimate_length(),
            Bezier::Bezier3(b) => b.estimate_length(),
        }
    }

    fn first_point(&self) -> T {
        match self {
            Bezier::Bezier0(b) => b.first_point(),
            Bezier::Bezier1(b) => b.first_point(),
            Bezier::Bezier2(b) => b.first_point(),
            Bezier::Bezier3(b) => b.first_point(),
        }
    }

    fn last_point(&self) -> T {
        match self {
            Bezier::Bezier0(b) => b.last_point(),
            Bezier::Bezier1(b) => b.last_point(),
            Bezier::Bezier2(b) => b.last_point(),
            Bezier::Bezier3(b) => b.last_point(),
        }
    }
}

impl<T: Distance + Mix + Clone + Debug> Debug for Bezier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bezier::Bezier0(b) => write!(f, "Bezier({:?})", b),
            Bezier::Bezier1(b) => write!(f, "Bezier({:?})", b),
            Bezier::Bezier2(b) => write!(f, "Bezier({:?})", b),
            Bezier::Bezier3(b) => write!(f, "Bezier({:?})", b),
        }
    }
}

impl<T: Distance + Mix + Clone + Copy> Copy for Bezier<T> {}

impl<T: Distance + Mix + Clone> From<Bezier0<T>> for Bezier<T> {
    fn from(b: Bezier0<T>) -> Self {
        Bezier::Bezier0(b)
    }
}

impl<T: Distance + Mix + Clone> From<Bezier1<T>> for Bezier<T> {
    fn from(b: Bezier1<T>) -> Self {
        Bezier::Bezier1(b)
    }
}

impl<T: Distance + Mix + Clone> From<Bezier2<T>> for Bezier<T> {
    fn from(b: Bezier2<T>) -> Self {
        Bezier::Bezier2(b)
    }
}

impl<T: Distance + Mix + Clone> From<Bezier3<T>> for Bezier<T> {
    fn from(b: Bezier3<T>) -> Self {
        Bezier::Bezier3(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezier0() {
        let b = Bezier0::new(0.0);
        assert_eq!(b.value_at(0.0), 0.0);
        assert_eq!(b.value_at(0.5), 0.0);
        assert_eq!(b.value_at(1.0), 0.0);
        assert_eq!(b.estimate_length(), 0.0);
    }

    #[test]
    fn test_bezier1() {
        let b = Bezier1::new(0.0, 1.0);
        assert_eq!(b.value_at(0.0), 0.0);
        assert_eq!(b.value_at(0.5), 0.5);
        assert_eq!(b.value_at(1.0), 1.0);
        assert_eq!(b.estimate_length(), 1.0);
    }

    #[test]
    fn test_bezier2() {
        let b = Bezier2::new(0.0, 1.5, 2.0);
        assert_eq!(b.value_at(0.0), 0.0);
        assert_eq!(b.value_at(0.5), 1.25);
        assert_eq!(b.value_at(1.0), 2.0);
        assert_eq!(b.estimate_length(), 2.0);
    }

    #[test]
    fn test_bezier3() {
        let b = Bezier3::new(0.0, 1.5, 2.0, 4.0);
        assert_eq!(b.value_at(0.0), 0.0);
        assert_eq!(b.value_at(0.5), 1.8125);
        assert_eq!(b.value_at(1.0), 4.0);
        assert_eq!(b.estimate_length(), 4.0);
    }
}
