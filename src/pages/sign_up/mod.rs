use crate::api::sign_up_api;
use crate::components::nav::NavComponent;
use crate::router::Route;
use crate::store::UserStore;
use log::info;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, FocusEvent, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;
use yewdux::dispatch::{Dispatch, Dispatcher};
use yewdux::prelude::BasicStore;

pub enum SignUpMessage {
    SignUp,
    Success(String),
    InputUsername(String),
    InputPassword(String),
    InputPasswordRepeat(String),
    UserState(Rc<UserStore>),
}

pub enum Stages {
    SignUp,
    Success,
}

pub struct SignUpPage {
    username: String,
    password: String,
    password_repeat: String,
    dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
    stage: Stages,
}

impl Component for SignUpPage {
    type Message = SignUpMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(SignUpMessage::UserState));
        Self {
            username: "".to_string(),
            password: "".to_string(),
            password_repeat: "".to_string(),
            stage: Stages::SignUp,
            dispatch,
            state: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignUpMessage::InputUsername(username) => {
                self.username = username;
                true
            }
            SignUpMessage::InputPassword(password) => {
                self.password = password;
                true
            }
            SignUpMessage::InputPasswordRepeat(password) => {
                self.password_repeat = password;
                true
            }
            SignUpMessage::SignUp => {
                let username = self.username.clone();
                let password = self.password.clone();
                let eq = password.eq(&self.password_repeat);
                if !eq {
                    return false;
                }
                info!("username: {}; password: {}", username, password);
                ctx.link().send_future(async {
                    match sign_up_api(username, password).await {
                        Ok(data) => SignUpMessage::Success(data.user_id),
                        Err(_) => SignUpMessage::Success("error".to_string()),
                    }
                });
                false
            }
            SignUpMessage::Success(user_id) => {
                info!("user_id {}", user_id);
                self.dispatch.reduce(|s| s.user_id = user_id);
                self.stage = Stages::Success;
                true
            }
            SignUpMessage::UserState(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change = |e: FocusEvent| e.prevent_default();
        match self.stage {
            Stages::SignUp => {
                html!(
                    <main>
                        <NavComponent/>
                        <div class="form-container center">
                            <form class="form form-auth" onsubmit={change}>
                                {self.get_header()}
                                {self.html_input_username(ctx)}
                                {self.html_input_password(ctx)}
                                {self.html_input_repeat_password(ctx)}
                                {self.html_button_signup(ctx)}
                            </form>
                        </div>
                    </main>
                )
            }
            Stages::Success => {
                html!(<Redirect<Route> to={Route::Home}/>)
            }
        }
    }
}

impl SignUpPage {
    fn html_button_signup(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div class="form-element column-direction center">
                <button cursor="pointer" class="primary-button" onclick={ctx.link().callback(|_| SignUpMessage::SignUp)}>
                    { "SignUp" }
                </button>
            </div>
        )
    }

    fn html_input_username(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignUpMessage::InputUsername(input.value()))
        });
        html! {
            <div class="form-element column-direction center">
                <label class="primary-input-label" for="username-input-signup">
                    { "Username:" }
                </label>
                <input class="primary-input" onchange={change}
                    id="username-input-signup"
                    type="text"
                />
            </div>
        }
    }

    fn html_input_password(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignUpMessage::InputPassword(input.value()))
        });
        html! {
            <div class="form-element column-direction center">
                <label class="primary-input-label" for="password-input-signup">
                    { "Password:" }
                </label>
                <input class="primary-input" onchange={change}
                    id="password-input-signup"
                    type="password"
                />
            </div>
        }
    }

    fn html_input_repeat_password(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignUpMessage::InputPasswordRepeat(input.value()))
        });
        html! {
            <div class="form-element column-direction center">
                <label class="primary-input-label" for="password-input-signup-repeat">
                    { "Password repeat:" }
                </label>
                <input class="primary-input" onchange={change}
                    id="password-input-signup-repeat"
                    type="password"
                />
            </div>
        }
    }

    fn get_header(&self) -> Html {
        html!(
            <h3 class="form-element column-direction center">{"Please, sign up"}</h3>
        )
    }
}
