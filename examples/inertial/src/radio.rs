use crate::mdl;
use std::marker::PhantomData;
use uuid::Uuid;
use web_sys::Element;
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
        let node = self.node_ref.cast::<Element>().unwrap();
        if first_render {
            mdl::upgrade_element(&node);
        } else {
            mdl::set_radio_checked(&node, ctx.props().checked);
        }
    }
}
