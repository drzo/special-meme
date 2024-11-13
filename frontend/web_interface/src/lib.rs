use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use log::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub user: String,
    pub message: String,
}

pub enum Msg {
    Send,
    Receive(ChatMessage),
    UpdateInput(String),
    Error(String),
}

pub struct Model {
    messages: Vec<ChatMessage>,
    input: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Send => {
                let message = ChatMessage {
                    user: "User".to_string(),
                    message: self.input.clone(),
                };
                self.messages.push(message.clone());
                self.input.clear();

                let link = ctx.link().clone();
                spawn_local(async move {
                    match Request::post("https://24be1794-5ca5-4650-b243-5a7fe7a9d9fb-00-3209duemqx6n2.janeway.replit.dev/api/chat")
                        .header("Content-Type", "application/json")
                        .json(&message)
                    {
                        Ok(request) => {
                            match request.send().await {
                                Ok(response) => {
                                    if response.ok() {
                                        match response.json::<ChatMessage>().await {
                                            Ok(result) => link.send_message(Msg::Receive(result)),
                                            Err(e) => {
                                                error!("Failed to parse response: {}", e);
                                                link.send_message(Msg::Error("Failed to parse response from server.".to_string()));
                                            }
                                        }
                                    } else {
                                        error!("Server error: {}", response.status());
                                        link.send_message(Msg::Error(format!("Server error: {}", response.status())));
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to send message: {}", e);
                                    link.send_message(Msg::Error("Failed to send message. Please try again.".to_string()));
                                },
                            }
                        },
                        Err(e) => {
                            error!("Failed to create request: {}", e);
                            link.send_message(Msg::Error("Failed to create request.".to_string()));
                        },
                    }
                });

                true
            }
            Msg::Receive(message) => {
                self.messages.push(message);
                true
            }
            Msg::UpdateInput(value) => {
                self.input = value;
                false
            }
            Msg::Error(error) => {
                self.messages.push(ChatMessage {
                    user: "System".to_string(),
                    message: error,
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::Send
        });

        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInput(input.value())
        });

        html! {
            <div class="container mt-5">
                <h1 class="mb-4">{"CogRox - Rusty Chatbot"}</h1>
                <div class="card">
                    <div class="card-body" style="height: 400px; overflow-y: auto;">
                        {for self.messages.iter().map(|msg| {
                            html! {
                                <p><strong>{&msg.user}{"："}</strong>{&msg.message}</p>
                            }
                        })}
                    </div>
                    <div class="card-footer">
                        <form {onsubmit}>
                            <div class="input-group">
                                <input
                                    type="text"
                                    class="form-control"
                                    placeholder="Type your message..."
                                    value={self.input.clone()}
                                    {oninput}
                                />
                                <button type="submit" class="btn btn-primary">{"Send"}</button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Model />
    }
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}
