use crate::circle::Circle;
use crate::dom::body;
use euclid::default::Point2D;

pub struct CircleList {
    circles: Vec<Circle>,
}

impl CircleList {
    pub fn new(items_count: usize, step_delay: f64) -> Self {
        CircleList {
            circles: (0..items_count)
                .map(|i| Circle::new(body(), i as f64 * step_delay))
                .collect(),
        }
    }

    pub fn set_mouse_position(&mut self, time: f64, position: Point2D<f32>) {
        self.circles
            .iter_mut()
            .for_each(|circle| circle.set_position(time, position));
    }

    pub fn update(&self, time: f64) {
        self.circles.iter().for_each(|circle| circle.update(time));
    }
}
