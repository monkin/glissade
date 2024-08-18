use crate::{Distance, Mix};
use std::fmt::Debug;

#[derive(Clone)]
pub(crate) struct Poly<T: Mix + Distance + Clone> {
    points: Vec<T>,
    offsets: Vec<f32>,
}

impl<T: Mix + Distance + Clone + Debug> Debug for Poly<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Poly")
            .field("points", &self.points)
            .field("offsets", &self.offsets)
            .finish()
    }
}

impl<T: Mix + Distance + Clone + PartialEq> PartialEq for Poly<T> {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl<T: Mix + Distance + Clone + Eq> Eq for Poly<T> {}

impl<T: Mix + Distance + Clone> Poly<T> {
    pub fn new(points: Vec<T>) -> Self {
        assert!(!points.is_empty());
        Self {
            offsets: points
                .windows(2)
                .map(|points| points[0].clone().distance(points[1].clone()))
                .fold(
                    {
                        let mut result = Vec::with_capacity(points.len());
                        result.push(0.0);
                        result
                    },
                    |mut acc, w| {
                        acc.push(acc.last().copied().unwrap_or_default() + w);
                        acc
                    },
                ),
            points,
        }
    }

    /// Returns the value at the given time `t` in the range [0.0, 1.0].
    pub fn value_at(&self, t: f32) -> T {
        let offset = self.length() * t.clamp(0.0, 1.0);

        let mut i1 = 0;
        let mut i2 = self.offsets.len() - 1;
        while i2 - i1 > 1 {
            let i = (i1 + i2) >> 1;
            if offset > self.offsets[i] {
                i1 = i;
            } else {
                i2 = i;
            }
        }

        let o1 = self.offsets[i1];
        let o2 = self.offsets[i2];

        let f = (offset - o1) / (o2 - o1);

        self.points[i1].clone().mix(self.points[i2].clone(), f)
    }

    pub(self) fn length(&self) -> f32 {
        self.offsets.last().copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let poly = Poly::new(vec![0.0, 1.0, 2.0]);
        assert_eq!(poly.value_at(0.0), 0.0);
        assert_eq!(poly.value_at(0.25), 0.5);
        assert_eq!(poly.value_at(0.5), 1.0);
        assert_eq!(poly.value_at(0.75), 1.5);
        assert_eq!(poly.value_at(1.0), 2.0);
    }

    #[test]
    fn test_debug() {
        let poly = Poly::new(vec![(0.0, 0.0), (2.0, 0.0), (2.0, 8.0)]);
        assert_eq!(poly.value_at(0.5), (2.0, 3.0));
        assert_eq!(poly.value_at(0.75), (2.0, 5.5));
    }
}
