use crate::radio::Radio;
use std::ops::Deref;
use strum::IntoEnumIterator;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EnumInputProps<T: Clone + PartialEq + ToString + IntoEnumIterator + 'static> {
    pub label: String,
    pub value: T,
    #[prop_or_default]
    pub on_change: Callback<T>,
}

#[function_component]
pub fn EnumInput<T: Clone + PartialEq + ToString + IntoEnumIterator + 'static>(
    props: &EnumInputProps<T>,
) -> Html {
    let on_change = &props.on_change;
    let value = props.value.clone();
    let label = props.label.clone();

    let id = use_memo(&(), |_| Uuid::new_v4().to_string())
        .deref()
        .clone();

    html! {
        <fieldset data-value={value.to_string()}>
            <legend class={classes!("mdl-color-text--grey-600")}>{ label }</legend>
            {
                T::iter().map(|variant| {
                    let variant = variant.clone();
                    html! {
                        <Radio<T>
                            label={variant.to_string()}
                            value={variant.clone()} name={id.clone()}
                            checked={value == variant}
                            on_change={on_change.clone()}
                        />
                    }
                }).collect::<Html>()
            }
        </fieldset>
    }
}
