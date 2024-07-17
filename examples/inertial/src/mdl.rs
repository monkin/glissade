use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = componentHandler, js_name = upgradeElement)]
    pub fn upgrade_element(element: &Element);

    #[wasm_bindgen(js_namespace = componentHandler, js_name = upgradeDom)]
    pub fn upgrade_dom();
}

#[wasm_bindgen(
    inline_js = "export function setRadioChecked(node, checked) { const c = node?.parentNode?.MaterialRadio; if (c) { c[checked ? 'check' : 'uncheck'](); }; }"
)]
extern "C" {
    #[wasm_bindgen(js_name = setRadioChecked)]
    pub fn set_radio_checked(checkbox: &Element, checked: bool);
}
