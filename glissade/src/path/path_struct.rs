use crate::path::bezier::Bezier;
use crate::path::Curve;
use crate::smooth_array::SmoothArray;
use crate::{Distance, Mix};
use std::fmt::Debug;

#[derive(Clone)]
pub struct Path<T: Mix + Distance + Clone> {
    segments: Vec<Bezier<T>>,
    indexes: SmoothArray,
}

impl<T: Mix + Distance + Clone + Debug> Debug for Path<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Path")
            .field("segments", &self.segments)
            .field("indexes", &self.indexes)
            .finish()
    }
}

impl<T: Mix + Distance + Clone> Path<T> {
    pub fn new(segments: Vec<Bezier<T>>, steps_count: usize, table_size: usize) -> Self {
        let (ranges, total_estimation) = segments
            .iter()
            .map(|segment| segment.estimate_length())
            .fold((Vec::with_capacity(segments.len()), 0.0), |mut acc, l| {
                let v1 = acc.1;
                let v2 = v1 + l;
                acc.0.push((v1, v2));
                acc.1 += v2;
                acc
            });

        let points = ranges.iter().zip(segments.iter()).enumerate().fold(
            Vec::with_capacity(steps_count),
            |mut points: Vec<(f32, f32, T)>, (segment_i, ((i1, i2), segment))| {
                if i1 == i2 {
                    return points;
                }

                let ii1 = ((i1 / total_estimation * ((steps_count - 1) as f32)).ceil() as usize)
                    .max(points.len());
                let ii2 = (i2 / total_estimation * ((steps_count - 1) as f32)).floor() as usize;
                let di = (i2 - i1) / total_estimation * ((steps_count - 1) as f32);
                for i in ii1..=ii2 {
                    let t = i as f32 / di;
                    let point = segment.value_at(t);

                    println!(
                        "i1: {:?}, i2: {:?}, i: {:?}, t: {:?}, ii1: {:?}, ii2: {:?}",
                        i1, i2, i, t, ii1, ii2
                    );

                    let offset = {
                        let point = point.clone();
                        points
                            .last()
                            .map(|p| p.1 + p.2.clone().distance(point))
                            .unwrap_or_default()
                    };
                    points.push((segment_i as f32 + t, offset, point));
                }
                points
            },
        );

        let total_length = points.last().map(|p| p.1).unwrap_or_default();
        let mut indexes = SmoothArray::new(table_size);
        points.windows(2).for_each(|window| {
            let (i1, offset1, _) = window[0];
            let (i2, offset2, _) = window[1];
            indexes.line((offset1 / total_length, i1), (offset2 / total_length, i2));
        });

        Self { segments, indexes }
    }

    pub fn value_at(&self, i: f32) -> T {
        let i = self.indexes.value_at(i);
        let segment_i = (i.floor() as usize).min(self.segments.len() - 1);
        let segment = &self.segments[segment_i];
        let t = i - segment_i as f32;
        segment.value_at(t)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Distance, PathBuilder};

    const TEST_PRECISION: f32 = 1.0e-5;

    fn close_to<T: Distance>(v1: T, v2: T) -> bool {
        v1.distance(v2) < TEST_PRECISION
    }

    #[test]
    fn test_path() {
        let path = PathBuilder::new((0.0, 0.0))
            .line_to((10.0, 10.0))
            .build(30, 30);

        assert_eq!(path.value_at(0.0), (0.0, 0.0));
        assert!(close_to(path.value_at(0.5), (5.0, 5.0)));
        assert_eq!(path.value_at(1.0), (10.0, 10.0));
    }
}
