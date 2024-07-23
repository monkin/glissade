use crate::dom::document;
use euclid::default::Point2D;
use glissade::{Animated, Inertial};
use web_sys::{Element, HtmlElement};

pub struct Circle {
    node: Element,
    delay: f64,
    position: Option<Inertial<Point2D<f32>, f64>>,
}

impl Circle {
    pub fn new(parent: HtmlElement, delay: f64) -> Self {
        let node = document().create_element("div").unwrap();
        node.class_list().add_1("circle").unwrap();
        parent.append_child(&node).unwrap();

        Circle {
            node,
            delay,
            position: None,
        }
    }

    pub fn set_position(&mut self, time: f64, position: Point2D<f32>) {
        let delay: f64 = self.delay;

        self.position = self
            .position
            .take()
            .map(|inertial| inertial.clone().go_to(position, time, delay))
            .or_else(|| Some(position.into()));
    }

    pub fn update(&self, time: f64) {
        if let Some(position) = self.position.as_ref() {
            let position = position.get(time);
            self.node
                .set_attribute(
                    "style",
                    &format!(
                        "transform: translate(-50%, -50%) translate({:.2}px, {:.2}px);",
                        position.x, position.y
                    ),
                )
                .unwrap();
        }
    }
}

impl Drop for Circle {
    fn drop(&mut self) {
        self.node.remove();
    }
}
