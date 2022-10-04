// use serde::{Serialize, Deserialize};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
// use material_yew::button::MatButton;
use wasm_bindgen::JsValue;
use web_sys::console;
use num_format::{Locale, ToFormattedString};

enum Msg {
    AddOne,
    PostUser
}

struct Model {
    value: i64,
    users_signed_up: u32,
    questions_answered: u32,
    average_score: f32
}

// #[derive(serde::Serialize,serde::Deserialize)]
// struct Test {
//     a: String,
//     b: u32
// }

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0, users_signed_up: 152223,questions_answered:14222321,average_score:78.5f32 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                spawn_local(async {
                    #[derive(Debug, Serialize, Deserialize)]
                    struct SomeData {
                        a: u32,
                    }

                    let response = reqwest::Client::new()
                        .get("http://127.0.0.1:8081/")
                        .send()
                        .await
                        .unwrap();
                    let js_value = JsValue::from_str(&format!("{:?}", response));
                    console::log_1(&js_value);
                    let json = response.json::<SomeData>().await.unwrap();
                    let js_value = JsValue::from_str(&format!("{:?}", json));
                    console::log_1(&js_value);
                });
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::PostUser => {
                let js_value = JsValue::from_str(&"post user triggered");
                console::log_1(&js_value);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <div class="container">
                    <div class="row" style="margin: 150px 0px 150px 0px;">
                        <h2 style="text-align: center;">{"Maths Site"}</h2>
                        <p style="text-align: center; font-size: 20px;">{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent sed lobortis ipsum. Cras non eros viverra lectus finibus consequat sit amet eu mauris. In non lacus tortor. Vestibulum nisl metus."}</p>
                    </div>
                    <div class="row">
                        <p>{ self.value }</p>
                        <a class="waves-effect waves-light btn" onclick={link.callback(|_| Msg::AddOne)}>{"button"}</a>
                    </div>
                    <div class="row">
                        <div class="col s12 l6">
                            <h4>{"A better way to practice maths"}</h4>
                            <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla eu dolor cursus, dapibus diam in, sodales odio. Pellentesque non ex dolor. Nulla mollis eleifend elit, ac volutpat nisi varius vel. Proin imperdiet, tellus id fermentum sagittis, nisl lectus hendrerit est."}</p>
                        </div>
                        <div class="col s12 l6">
                            <h4>{"Register or Login"}</h4>
                            <p>{"No need to copy and paste that password, and use whatever you want. We trust you."}</p>
                            <div>
                                <div class="input-field">
                                    <i class="material-icons prefix">{"face"}</i>
                                    <input id="email" type="email" class="validate"/>
                                    <label for="email">{"Email"}</label>
                                </div>
                                <div class="input-field">
                                    <i class="material-icons prefix">{"lock_outline"}</i>
                                    <input id="password" type="password" class="validate"/>
                                    <label for="password">{"Password"}</label>
                                </div>
                                <div class="center-align">
                                    <div>
                                        <button class="btn waves-effect waves-light" onclick={link.callback(|_| Msg::PostUser)}>
                                            {"Login / Register"}
                                            <i class="material-icons right">{"lock_open"}</i>
                                        </button>
                                    </div>
                                    <div>
                                        <a class="waves-effect waves-teal btn-flat" href="#" id="recover-link" style="border-radius: 0px 0px 10px 10px;">{"Recover password"}</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="col s12 l6">
                            <h4>{"Statistics"}</h4>
                            <p>
                                { "With " }
                                <b>{ self.users_signed_up.to_formatted_string(&Locale::en) }{ " users signed-up " }</b> 
                                { "and " }
                                <b>{ self.questions_answered.to_formatted_string(&Locale::en) }{ " questions answered " }</b>
                                { "with an " }
                                <b>{"average score of "}{ format!("{:.1}",self.average_score) }{ "%" }</b>
                                { " we're helping people practice." }
                            </p>
                            <canvas></canvas>
                        </div>
                        <div class="col s12 l6">
                            <h4>{"Translation"}</h4>
                            <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur vestibulum augue nunc, nec egestas augue bibendum a. In a ante."}</p>
                            <img alt="Example image of translation from handwritten maths to LaTex" src="translation-example.jpg" style="max-width: 100%; max-height: 200px;"/>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
// #[actix_web::main]
fn main() {
    yew::start_app::<Model>();
}
