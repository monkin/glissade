mod animated_view;
mod animation_loop;
mod button;
mod enum_input;
mod mdl;
mod radio;
mod use_inertial;

use crate::animated_view::{AnimatedView, AnimatedViewProps};
use crate::button::Button;
use crate::enum_input::EnumInput;
use cgmath::Vector2;
use palette::LinSrgb;
use rand::rngs::ThreadRng;
use rand::Rng;
use yew::prelude::*;

#[derive(Clone, PartialEq, Copy, Default, strum_macros::Display, strum_macros::EnumIter)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

impl Size {
    fn random(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..3) {
            0 => Size::Small,
            1 => Size::Medium,
            _ => Size::Large,
        }
    }
}

impl From<Size> for f32 {
    fn from(value: Size) -> Self {
        match value {
            Size::Small => 1.0,
            Size::Medium => 1.5,
            Size::Large => 2.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, strum_macros::Display, strum_macros::EnumIter)]
pub enum Shape {
    #[default]
    Circle,
    Square,
}

impl Shape {
    fn random(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..2) {
            0 => Shape::Circle,
            _ => Shape::Square,
        }
    }
}

impl From<Shape> for f32 {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Circle => 1.0,
            Shape::Square => 0.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, strum_macros::Display, strum_macros::EnumIter)]
pub enum HorizontalPosition {
    Left,
    #[default]
    Center,
    Right,
}

impl From<HorizontalPosition> for f32 {
    fn from(value: HorizontalPosition) -> Self {
        match value {
            HorizontalPosition::Left => 0.0,
            HorizontalPosition::Center => 0.5,
            HorizontalPosition::Right => 1.0,
        }
    }
}

impl HorizontalPosition {
    fn random(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..3) {
            0 => HorizontalPosition::Left,
            1 => HorizontalPosition::Center,
            _ => HorizontalPosition::Right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, strum_macros::Display, strum_macros::EnumIter)]
pub enum VerticalPosition {
    Top,
    #[default]
    Middle,
    Bottom,
}

impl VerticalPosition {
    fn random(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..3) {
            0 => VerticalPosition::Top,
            1 => VerticalPosition::Middle,
            _ => VerticalPosition::Bottom,
        }
    }
}

impl From<VerticalPosition> for f32 {
    fn from(value: VerticalPosition) -> Self {
        match value {
            VerticalPosition::Top => 0.0,
            VerticalPosition::Middle => 0.5,
            VerticalPosition::Bottom => 1.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, strum_macros::Display, strum_macros::EnumIter)]
pub enum Color {
    Red,
    Green,
    #[default]
    Blue,
    Gray,
    Black,
}

impl Color {
    fn random(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..5) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Gray,
            _ => Color::Black,
        }
    }
}

impl From<Color> for LinSrgb {
    fn from(value: Color) -> Self {
        match value {
            Color::Red => LinSrgb::new(1.0, 0.0, 0.0),
            Color::Green => LinSrgb::new(0.0, 1.0, 0.0),
            Color::Blue => LinSrgb::new(0.0, 0.0, 1.0),
            Color::Gray => LinSrgb::new(0.5, 0.5, 0.5),
            Color::Black => LinSrgb::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
struct App {
    size: Size,
    shape: Shape,
    horizontal_position: HorizontalPosition,
    vertical_position: VerticalPosition,
    color: Color,
}

impl App {
    fn random() -> App {
        let mut rng = rand::thread_rng();
        App {
            size: Size::random(&mut rng),
            shape: Shape::random(&mut rng),
            horizontal_position: HorizontalPosition::random(&mut rng),
            vertical_position: VerticalPosition::random(&mut rng),
            color: Color::random(&mut rng),
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
enum AppMessage {
    Size(Size),
    Shape(Shape),
    HorizontalPosition(HorizontalPosition),
    VerticalPosition(VerticalPosition),
    Color(Color),
    Random,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Size(size) => {
                self.size = size;
            }
            AppMessage::Shape(shape) => {
                self.shape = shape;
            }
            AppMessage::HorizontalPosition(horizontal_position) => {
                self.horizontal_position = horizontal_position;
            }
            AppMessage::VerticalPosition(vertical_position) => {
                self.vertical_position = vertical_position;
            }
            AppMessage::Color(color) => {
                self.color = color;
            }
            AppMessage::Random => {
                *self = App::random();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_shape_change = ctx.link().callback(AppMessage::Shape);
        let on_size_change = ctx.link().callback(AppMessage::Size);
        let on_horizontal_position_change = ctx.link().callback(AppMessage::HorizontalPosition);
        let on_vertical_position_change = ctx.link().callback(AppMessage::VerticalPosition);
        let on_color_change = ctx.link().callback(AppMessage::Color);

        let animated_props: AnimatedViewProps = AnimatedViewProps {
            size: self.size.into(),
            border_radius: self.shape.into(),
            position: Vector2::new(
                self.horizontal_position.into(),
                self.vertical_position.into(),
            ),
            color: self.color.into(),
        };

        html! {
            <div class={classes!("mdl-layout", "mdl-js-layout", "mdl-layout--fixed-drawer")}>
                <div class={classes!("mdl-layout__drawer", "control-panel")}>
                    <Button label="Random" on_click={ctx.link().callback(|_| AppMessage::Random)} />
                    <EnumInput<Shape> label="Shape:" value={self.shape} on_change={on_shape_change} />
                    <EnumInput<Size> label="Size:" value={self.size} on_change={on_size_change} />
                    <EnumInput<HorizontalPosition> label="Horizontal Position:" value={self.horizontal_position} on_change={on_horizontal_position_change} />
                    <EnumInput<VerticalPosition> label="Vertical Position:" value={self.vertical_position} on_change={on_vertical_position_change} />
                    <EnumInput<Color> label="Color:" value={self.color} on_change={on_color_change} />
                </div>
                <main class={classes!("mdl-layout__content")}>
                    <AnimatedView ..animated_props />
                </main>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            mdl::upgrade_dom();
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
