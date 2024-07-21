mod animation_loop;
mod circle;
mod circle_list;
mod dom;

use crate::animation_loop::AnimationLoop;
use crate::circle_list::CircleList;
use crate::dom::{now, MousePositionListener};
use euclid::default::Point2D;
use std::cell::RefCell;
use std::rc::Rc;

const ITEMS_COUNT: usize = 30;

/// 100ms for the first item, 200ms for the second, etc.
const STEP_DELAY: f64 = 150.0;

const DEBOUNCE: f64 = 100.0;

struct App {
    last_update: f64,
    mouse_position: Option<Point2D<f32>>,
    circle_list: CircleList,
}

impl App {
    fn new() -> Self {
        App {
            last_update: 0.0,
            mouse_position: None,
            circle_list: CircleList::new(ITEMS_COUNT, STEP_DELAY),
        }
    }

    fn set_mouse_position(&mut self, position: Point2D<f32>) {
        self.mouse_position = Some(position);
    }

    fn update(&mut self, time: f64) {
        if let Some(mouse_position) = self.mouse_position {
            if time - self.last_update > DEBOUNCE {
                self.last_update = time;
            }
            self.circle_list.set_mouse_position(time, mouse_position);
            self.mouse_position = None;
        }

        self.circle_list.update(time);
    }
}

fn main() {
    let app = Rc::new(RefCell::new(App::new()));
    let animation_loop = AnimationLoop::new({
        let app = app.clone();
        move || app.borrow_mut().update(now())
    });
    let mouse_position_listener = MousePositionListener::new({
        let app = app.clone();
        move |x, y| {
            app.borrow_mut()
                .set_mouse_position(Point2D::new(x as f32, y as f32))
        }
    });

    std::mem::forget(Box::new((app, animation_loop, mouse_position_listener)));
}
