mod env;
mod status;

use crate::status::StatusKind;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

fn on_input(
    text: UseStateHandle<String>,
    label: UseStateHandle<StatusKind>,
) -> impl Fn(InputEvent) {
    move |event: InputEvent| {
        if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
            text.set(input.value())
        } else {
            log::error!("{}", StatusKind::InvalidInput);
            label.set(StatusKind::InvalidInput)
        }
    }
}

fn on_submit(
    text: UseStateHandle<String>,
    label: UseStateHandle<StatusKind>,
) -> impl Fn(SubmitEvent) {
    move |event: SubmitEvent| {
        // prevent page reload
        event.prevent_default();

        // current text input
        let text = text.trim().to_string();

        // discontinue if the input is empty
        if text.is_empty() {
            return label.set(StatusKind::InvalidInput);
        }

        // only communicate with server if input exists
        let (mut write, mut read) = match WebSocket::open(env::WLRS_WEBSOCKET_ADDR) {
            Ok(s) => s.split(),
            Err(e) => {
                log::error!("{}", e);
                label.set(StatusKind::Connection);
                return;
            }
        };

        let label = label.clone();
        spawn_local(async move {
            // send 'username' to server
            if let Err(e) = write.send(Message::Text(text)).await {
                log::error!("{}", e);
                label.set(StatusKind::Connection);
                return;
            }

            // receive server response
            let res = if let Some(res) = read.next().await {
                res
            } else {
                log::error!("{}", StatusKind::Unexpected);
                label.set(StatusKind::Unexpected);
                return;
            };

            // parse response
            match res {
                Ok(res) => {
                    if let Message::Bytes(bytes) = res {
                        let kind = StatusKind::from_u8(bytes.first().unwrap_or(&4));
                        label.set(kind)
                    } else {
                        log::error!("{}", StatusKind::Unexpected)
                    }
                }
                Err(e) => {
                    log::error!("{}", e);
                    label.set(StatusKind::Unexpected)
                }
            }
        });
    }
}

#[allow(non_snake_case)]
#[function_component]
fn App() -> Html {
    let text = use_state_eq(String::new);
    let label_opt = use_state_eq(StatusKind::default);

    let oninput = on_input(text.clone(), label_opt.clone());
    let onsubmit = on_submit(text.clone(), label_opt.clone());

    html! {
        <div class="outer" action="#">
            <div class="inner">
                <form {onsubmit}>
                    <input {oninput} type="text" placeholder="Enter Minecraft Player Name"/>
                    <input type="submit"/>
                </form>
                if label_opt.is_new() {
                    { label_opt.as_html() }
                }
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
