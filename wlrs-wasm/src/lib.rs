#[cfg(feature = "debug")]
mod utils;

use dotenv::{dotenv, var};
use wasm_bindgen::prelude::*;
use web_sys::{
    js_sys::{ArrayBuffer, Uint8Array},
    window, BinaryType, Event, HtmlElement, HtmlInputElement, MessageEvent, WebSocket,
};

enum Status {
    Success,
    Warning,
    Failure,
}

struct HtmlElementStyle;

impl HtmlElementStyle {
    fn set_color(e: &HtmlElement, status: Status) {
        drop(e.set_attribute(
            "color",
            match status {
                Status::Success => "#4dff4d",
                Status::Warning => "#ffe400",
                Status::Failure => "#ff5050",
            },
        ))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn handle_input(input: String, p: HtmlElement, addr: &str) {
    if input.is_empty() {
        return p.set_text_content(Some("Please provide a valid username."));
    }

    let ws = if let Ok(ws) = WebSocket::new(addr) {
        ws
    } else {
        p.set_text_content(Some("Failed to initiate websocket."));
        HtmlElementStyle::set_color(&p, Status::Failure);
        return;
    };
    ws.set_binary_type(BinaryType::Arraybuffer);

    {
        let p_clone = p.clone();
        let ws_onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            let res = e
                .data()
                .dyn_into::<ArrayBuffer>()
                .map(|buf| Uint8Array::new(&buf).to_vec())
                .unwrap_or(vec![4]);

            let (status, msg) = match res.first().unwrap_or(&4) {
                0 => (Status::Failure, "Minecraft server is down"),
                1 => (Status::Warning, "Player doesn't exist"),
                2 => (Status::Success, "Already whitelisted"),
                3 => (Status::Success, "Success"),
                _ => (Status::Failure, "Unexpected server response"),
            };

            p_clone.set_text_content(Some(msg));
            HtmlElementStyle::set_color(&p_clone, status);
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

    {
        let ws_clone = ws.clone();
        let ws_onerror = Closure::wrap(Box::new(move || {
            p.set_text_content(Some("Issue(s) communicating with server."));
            HtmlElementStyle::set_color(&p, Status::Failure);
            ws_clone.close().unwrap_or_default();
        }) as Box<dyn Fn()>);
        ws.set_onerror(Some(ws_onerror.as_ref().unchecked_ref()));
        ws_onerror.forget();
    }
}

#[wasm_bindgen(start)]
fn start() {
    #[cfg(feature = "debug")]
    utils::set_panic_hook();

    dotenv().expect("Failed to get environmental variables.");
    let addr = var("WLRS_WEBSOCKET_ADDR").unwrap();

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
        let value =
            Closure::wrap(
                Box::new(move |_: Event| handle_input(input.value(), p.clone(), &addr))
                    as Box<dyn Fn(_)>,
            );
        submit.set_onclick(Some(value.as_ref().unchecked_ref()));
        value.forget();
    }
}
