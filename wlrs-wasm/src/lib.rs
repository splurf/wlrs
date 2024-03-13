mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::{
    js_sys::{ArrayBuffer, Uint8Array},
    window, BinaryType, Event, HtmlElement, HtmlInputElement, MessageEvent, WebSocket,
};

fn handle_input(input: String, p: HtmlElement) {
    if input.is_empty() {
        return p.set_text_content(Some("Please provide a valid username."));
    }

    let ws = WebSocket::new("wss://mc.rustychads.com/api").unwrap();
    ws.set_binary_type(BinaryType::Arraybuffer);

    {
        let ws_onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            let buf = e.data().dyn_into::<ArrayBuffer>().unwrap();
            let data = Uint8Array::new(&buf).to_vec();

            let (text, color) = if data == [1] {
                ("Success", "#4dff4d")
            } else {
                ("Failed", "#ff5050")
            };

            p.set_text_content(Some(text));
            p.set_attribute("color", color).unwrap();
        }) as Box<dyn Fn(_)>);

        ws.set_onmessage(Some(ws_onmessage.as_ref().unchecked_ref()));
        ws_onmessage.forget();
    }

    {
        let ws_clone = ws.clone();
        let ws_onopen = Closure::wrap(Box::new(move || {
            ws_clone.send_with_str(input.as_str()).unwrap();
        }) as Box<dyn Fn()>);
        ws.set_onopen(Some(ws_onopen.as_ref().unchecked_ref()));
        ws_onopen.forget();
    }
}

#[wasm_bindgen(start)]
fn start() {
    set_panic_hook();

    let window = window().expect("Failed to get Window object.");
    let document = window.document().expect("Failed to get Document object.");

    let submit = document
        .get_element_by_id("submit")
        .expect("Failed to get input element.")
        .dyn_into::<HtmlInputElement>()
        .expect("Failed to cast to 'HtmlInputElement'.");

    let input = document
        .get_element_by_id("player")
        .expect("Failed to get input element.")
        .dyn_into::<HtmlInputElement>()
        .expect("Failed to cast to 'HtmlInputElement'.");

    let p = document
        .get_element_by_id("result")
        .expect("Failed to get paragraph element.")
        .dyn_into::<HtmlElement>()
        .expect("Failed to cast to 'HtmlElement'.");

    {
        let value = Closure::wrap(
            Box::new(move |_: Event| handle_input(input.value(), p.clone())) as Box<dyn Fn(_)>,
        );
        submit.set_onclick(Some(value.as_ref().unchecked_ref()));
        value.forget();
    }
}
