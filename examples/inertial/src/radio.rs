use js_sys::{Function, Reflect};
use std::marker::PhantomData;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps<V: Clone + PartialEq + ToString + 'static> {
    pub label: String,
    pub name: String,
    pub value: V,
    pub checked: bool,
    #[prop_or_default]
    pub on_change: Callback<V>,
}

pub struct Radio<V: Clone + PartialEq + ToString + 'static> {
    id: String,
    phantom_data: PhantomData<V>,
    node_ref: NodeRef,
}

impl<V: Clone + PartialEq + ToString + 'static> Component for Radio<V> {
    type Message = ();
    type Properties = RadioProps<V>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            phantom_data: Default::default(),
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let id = self.id.clone();

        let value: V = props.value.clone();
        let label = props.label.clone();
        let name = props.name.clone();
        let checked = props.checked;

        let handle_change = ctx.link().callback({
            let value = value.clone();
            let on_change = props.on_change.clone();
            move |_| {
                on_change.emit(value.clone());
            }
        });

        html! {
            <label class={classes!("mdl-radio", "mdl-js-radio", "mdl-js-ripple-effect")} for={id.clone()}>
                <input
                    ref={self.node_ref.clone()}
                    type="radio"
                    id={id}
                    class={classes!("mdl-radio__button")}
                    name={name}
                    value={value.to_string()}
                    checked={checked}
                    onchange={handle_change}
                />
                <span>{ label }</span>
            </label>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let node = self.node_ref.cast::<HtmlElement>().unwrap();
        if first_render {
            let component_handler =
                Reflect::get(&window().unwrap(), &"componentHandler".into()).unwrap();
            let upgrade_element =
                Reflect::get(&component_handler, &"upgradeElement".into()).unwrap();
            let upgrade_element = upgrade_element.dyn_ref::<Function>().unwrap();
            upgrade_element.call1(&component_handler, &node).unwrap();
            return;
        }

        let parent = Reflect::get(&node, &"parentNode".into()).unwrap();
        let mdl = Reflect::get(&parent, &"MaterialRadio".into()).unwrap();
        let function_name = if ctx.props().checked {
            "check"
        } else {
            "uncheck"
        };

        let method_object = Reflect::get(&mdl, &function_name.into()).unwrap();
        let method = method_object.dyn_ref::<Function>().unwrap();
        method.call0(&mdl).unwrap();
    }
}
