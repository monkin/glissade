use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlElement, MouseEvent};

pub fn document() -> Document {
    window().unwrap().document().unwrap()
}

pub fn body() -> HtmlElement {
    document().body().unwrap()
}

pub fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

pub struct MousePositionListener {
    on_mouse_move: Closure<dyn FnMut(MouseEvent)>,
}

impl MousePositionListener {
    pub fn new<F>(mut callback: F) -> Self
    where
        F: FnMut(f64, f64) + 'static,
    {
        let on_mouse_move = Closure::new(move |event: MouseEvent| {
            let x = event.client_x() as f64;
            let y = event.client_y() as f64;
            callback(x, y);
        });

        window()
            .unwrap()
            .add_event_listener_with_callback("mousemove", on_mouse_move.as_ref().unchecked_ref())
            .unwrap();

        MousePositionListener { on_mouse_move }
    }
}

impl Drop for MousePositionListener {
    fn drop(&mut self) {
        window()
            .unwrap()
            .remove_event_listener_with_callback(
                "mousemove",
                self.on_mouse_move.as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}
