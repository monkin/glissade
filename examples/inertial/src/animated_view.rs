use crate::use_inertial::use_inertial;
use cgmath::Vector2;
use glissade::Mix;
use palette::{LinSrgb, Srgb};
use std::f32::consts::PI;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use web_time::Duration;
use yew::prelude::*;

const ANIMATION_DURATION: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, PartialEq, Debug, Mix, Properties)]
pub struct AnimatedViewProps {
    /// 1.0, 1.5, or 2.0
    pub size: f32,
    /// A value between 0.0 and 1.0
    pub border_radius: f32,
    /// X, and Y in range between 0.0 and 1.0
    /// 0, 0 - top left corner
    /// 1, 1 - bottom right corner
    pub position: Vector2<f32>,
    /// A color of the shape
    pub color: LinSrgb,
}

#[function_component]
pub fn AnimatedView(props: &AnimatedViewProps) -> Html {
    let animated = use_inertial(props, ANIMATION_DURATION);
    let node = use_node_ref();

    let element = node.cast::<HtmlElement>();

    let (width, height) = match &element {
        Some(element) => {
            let width = element.client_width() as f32;
            let height = element.client_height() as f32;
            (width, height)
        }
        None => (0.0, 0.0),
    };

    let trigger = use_force_update();

    use_effect_with((), {
        let trigger = trigger.clone();

        move |_| {
            let refresh: Closure<dyn Fn()> = Closure::new(move || trigger.force_update());

            let window = window().unwrap();

            let first_frame_id = window
                .request_animation_frame(refresh.as_ref().unchecked_ref())
                .unwrap();

            window
                .add_event_listener_with_callback("resize", refresh.as_ref().unchecked_ref())
                .unwrap();
            move || {
                window.cancel_animation_frame(first_frame_id).unwrap();
                window
                    .remove_event_listener_with_callback("resize", refresh.as_ref().unchecked_ref())
                    .unwrap();
            }
        }
    });

    let min = width.min(height);

    let size = animated.size / 8.0 * min * 1.0.mix(4.0 / PI, animated.border_radius);
    let size_str = format!("{}", size);

    let radius = animated.border_radius * size * 0.5;

    let x = animated.position.x * (width - size);
    let y = animated.position.y * (height - size);

    let color: Srgb<u8> = animated.color.into();
    let color = format!("rgb({}, {}, {})", color.red, color.green, color.blue);

    html! {
        <div class={classes!("animated-view")} ref={node}>
            <svg width={format!("{}", width)} height={format!("{}", height)}>
                <rect
                    x={format!("{}", x)}
                    y={format!("{}", y)}
                    width={size_str.clone()}
                    height={size_str}
                    rx={format!("{}", radius)}
                    fill={color}
                />
            </svg>
        </div>
    }
}
