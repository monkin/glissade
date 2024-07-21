use js_sys::Function;
use wasm_bindgen::prelude::*;
use yew::TearDown;

#[wasm_bindgen(
    inline_js = "export function animation_loop(callback) { let request_id = -1; function loop() { callback(); request_id = requestAnimationFrame(loop); }; loop(); return () => cancelAnimationFrame(request_id); }"
)]
extern "C" {
    fn animation_loop(callback: &Closure<dyn FnMut()>) -> Function;
}

pub struct AnimationLoop {
    callback: Box<Closure<dyn FnMut()>>,
    stop: Function,
}

impl AnimationLoop {
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let callback = Box::new(Closure::new(callback));
        AnimationLoop {
            stop: animation_loop(callback.as_ref()),
            callback,
        }
    }

    pub fn stop(&self) {
        self.stop.call0(&JsValue::NULL).unwrap();
    }
}

impl Drop for AnimationLoop {
    fn drop(&mut self) {
        self.stop();
        self.callback = Box::new(Closure::new(|| {}));
    }
}

impl TearDown for AnimationLoop {
    fn tear_down(self) {
        self.stop();
    }
}
