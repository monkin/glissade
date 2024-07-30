/// SmoothArray is a data structure that allows to interpolate values between data points.
/// Indexes are in range 0.0..=1.0.
#[derive(Clone, Debug, PartialEq)]
pub struct SmoothArray {
    data: Vec<f32>,
}

impl SmoothArray {
    pub fn with_steps_count(steps_count: usize) -> Self {
        Self {
            data: vec![0.0; steps_count],
        }
    }

    pub fn value_at(&self, i: f32) -> f32 {
        let i = i.clamp(0.0, 1.0) * (self.data.len() as f32 - 1.0);

        let f = i.fract();
        let i1 = i.floor() as usize;
        let i2 = i.ceil() as usize;

        let v1 = self.data[i1];
        let v2 = self.data[i2];

        v1 + (v2 - v1) * f
    }

    pub fn line(&mut self, (i1, v1): (f32, f32), (i2, v2): (f32, f32)) {
        let last_index = self.data.len() as f32 - 1.0;
        let i1 = i1 * last_index;
        let i2 = i2 * last_index;

        let idi = 1.0 / (i2 - i1);

        let mut i = i1.ceil();
        let max_i = i2.max(last_index);
        while i <= max_i {
            let f = (i - i1) * idi;
            let v = v1 * (1.0 - f) + v2 * f;
            self.data[i as usize] = v;
            i += 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth_array() {
        let mut array = SmoothArray::with_steps_count(10);
        array.line((0.0, 0.0), (1.0, 1.0));

        assert_eq!(array.value_at(0.0), 0.0);
        assert_eq!(array.value_at(0.5), 0.5);
        assert_eq!(array.value_at(1.0), 1.0);
    }
}
