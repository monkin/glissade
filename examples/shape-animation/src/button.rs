use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<()>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let on_click = props.on_click.clone();
    let label = props.label.clone();

    html! {
        <button
            class={classes!("mdl-button", "mdl-js-button", "mdl-button--raised", "mdl-js-ripple-effect")}
            onclick={Callback::from(move |_| on_click.emit(()))}
        >
            { label }
        </button>
    }
}
