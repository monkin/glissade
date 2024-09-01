use glam::Vec2;
use glissade::{keyframes, Animated, Distance, Easing, Keyframes, Mix};
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Debug, Mix, Default)]
struct PathItem {
    opacity: f32,
    position: Vec2,
}

impl Distance for PathItem {
    fn distance(self, other: Self) -> f32 {
        self.position.distance(other.position)
    }
}

impl PathItem {
    fn new(opacity: f32, position: Vec2) -> Self {
        Self { opacity, position }
    }
}

pub struct PathView {
    path: Box<dyn Animated<PathItem, Instant>>,
    point: Element,
}

const SVG_NS: Option<&str> = Some("http://www.w3.org/2000/svg");

impl PathView {
    pub fn new(
        title: &str,
        now: Instant,
        points: &[Vec2],
        parent: &Element,
    ) -> Result<Self, JsValue> {
        let document = parent.owner_document().unwrap();

        let container = {
            let node = document.create_element("li")?;
            parent.append_child(&node)?;
            node
        };

        let figure = {
            let node = document.create_element("figure")?;
            container.append_child(&node)?;
            node
        };

        let _header = {
            let node = document.create_element("figcaption")?;
            node.set_text_content(Some(title));
            figure.append_child(&node)?;
            node
        };

        let svg = {
            let node = document.create_element_ns(SVG_NS, "svg")?;
            node.set_attribute("width", "320")?;
            node.set_attribute("height", "240")?;
            node.set_attribute("viewBox", "-160 -120 320 240")?;
            figure.append_child(&node)?;
            node
        };

        let _polygon = {
            let node = document.create_element_ns(SVG_NS, "polygon")?;
            node.set_attribute(
                "points",
                &points
                    .iter()
                    .map(|p| format!("{:.2},{:.2}", p.x, p.y))
                    .collect::<Vec<_>>()
                    .join(" "),
            )?;
            svg.append_child(&node)?;
            node
        };
        let point = {
            let node = document.create_element_ns(SVG_NS, "circle")?;
            node.set_attribute("r", "6")?;
            svg.append_child(&node)?;
            node
        };

        let origin = points.first().copied().unwrap();

        // Create a keyframes animation
        let animation = keyframes::from(PathItem::new(1.0, origin))
            // blink two times
            .go_to(PathItem::new(0.0, origin), Duration::from_millis(400))
            .go_to(PathItem::new(1.0, origin), Duration::from_millis(400))
            .repeat_n(2.0)
            // move along the path
            .poly_to(
                points
                    .iter()
                    .map(|p| PathItem::new(1.0, *p))
                    .collect::<Vec<_>>(),
                Duration::from_secs(2),
                Easing::CubicInOut,
            )
            .repeat()
            .run(now);

        Ok(Self {
            path: Box::new(animation),
            point,
        })
    }

    pub fn update(&self, time: Instant) -> Result<(), JsValue> {
        // Get current animation position and opacity
        let item = self.path.as_ref().get(time);

        let position = item.position;
        let opacity = item.opacity;

        self.point
            .set_attribute("cx", &format!("{:.2}", position.x))?;
        self.point
            .set_attribute("cy", &format!("{:.2}", position.y))?;
        self.point
            .set_attribute("fill-opacity", &format!("{:.2}", opacity))?;
        Ok(())
    }
}
