use crate::animation_loop::AnimationLoop;
use crate::path_view::PathView;
use crate::paths::{lissajous, ngon, star};
use std::mem::forget;
use wasm_bindgen::JsValue;
use web_sys::{window, Element};
use web_time::Instant;

mod animation_loop;
mod path_view;
mod paths;

pub fn run_animations(container: &Element) -> Result<AnimationLoop, JsValue> {
    let node = {
        let node = container.owner_document().unwrap().create_element("ul")?;
        container.append_child(&node)?;
        node
    };

    let now = Instant::now();

    let items: Vec<PathView> = vec![
        PathView::new("Triangle", now, &ngon(100.0, 3).collect::<Vec<_>>(), &node)?,
        PathView::new("Square", now, &ngon(100.0, 4).collect::<Vec<_>>(), &node)?,
        PathView::new("Pentagon", now, &ngon(100.0, 5).collect::<Vec<_>>(), &node)?,
        PathView::new("Hexagon", now, &ngon(100.0, 6).collect::<Vec<_>>(), &node)?,
        PathView::new("Octagon", now, &ngon(100.0, 8).collect::<Vec<_>>(), &node)?,
        PathView::new("Circle", now, &ngon(100.0, 200).collect::<Vec<_>>(), &node)?,
        PathView::new("Star", now, &star(100.0, 5).collect::<Vec<_>>(), &node)?,
        PathView::new(
            "Lissajous 1:3",
            now,
            &lissajous(100.0, 500, 1, 3).collect::<Vec<_>>(),
            &node,
        )?,
        PathView::new(
            "Lissajous 3:4",
            now,
            &lissajous(100.0, 500, 3, 4).collect::<Vec<_>>(),
            &node,
        )?,
    ];

    Ok(AnimationLoop::new(move || {
        let now = Instant::now();
        for item in &items {
            item.update(now).unwrap();
        }
    }))
}

fn main() {
    let document = window().unwrap().document().unwrap();
    let app = Box::new(run_animations(&document.body().unwrap()).unwrap());
    forget(app);
}
