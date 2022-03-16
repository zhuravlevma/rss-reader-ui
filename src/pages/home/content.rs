use crate::api::get_content;
use crate::components::link::LinkComponent;
use crate::dto::ContentDto;
use crate::store::UserStore;
use gloo_utils;
use std::rc::Rc;
use web_sys::{window, Element};
use yew::{function_component, html, Component, Context, Html, Properties};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;

pub enum ContentMessage {
    UserState(Rc<UserStore>),
    SuccessContentNormal(Vec<ContentDto>),
    Next,
    Back,
}

pub struct ContentPage {
    _dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
    content: Vec<ContentDto>,
    start: u32,
    take: u32,
}

impl Component for ContentPage {
    type Message = ContentMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(ContentMessage::UserState));
        Self {
            _dispatch: dispatch,
            state: Default::default(),
            content: vec![],
            start: 0,
            take: 15,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ContentMessage::UserState(state) => {
                self.state = state;
                if self.state.token.is_empty() || self.state.token.eq("error") {
                    return true;
                }
                let token = self.state.token.clone();
                let start = self.start;
                let take = self.take;
                ctx.link().send_future(async move {
                    match get_content(token, start, take).await {
                        Ok(data) => ContentMessage::SuccessContentNormal(data),
                        Err(_) => ContentMessage::SuccessContentNormal(vec![]),
                    }
                });
                true
            }
            ContentMessage::Next => {
                self.start += 15;
                let take = self.take;
                let start = self.start;
                let token = self.state.token.clone();
                ctx.link().send_future(async move {
                    match get_content(token, start, take).await {
                        Ok(data) => ContentMessage::SuccessContentNormal(data),
                        Err(_) => ContentMessage::SuccessContentNormal(vec![]),
                    }
                });
                false
            }
            ContentMessage::Back => {
                self.start -= 15;
                let take = self.take;
                let start = self.start;
                let token = self.state.token.clone();
                ctx.link().send_future(async move {
                    match get_content(token, start, take).await {
                        Ok(data) => ContentMessage::SuccessContentNormal(data),
                        Err(_) => ContentMessage::SuccessContentNormal(vec![]),
                    }
                });
                false
            }
            ContentMessage::SuccessContentNormal(content) => {
                window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
                self.content = content;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
            <div class="content">
                <LinkComponent />
                <div class="container-content">
                    <ul class="content-list">
                        <ul>{self.get_content()}</ul>
                    </ul>
                    <div class="content-paging">
                        <button onclick={ctx.link().callback(|_| ContentMessage::Back)} type="button" class="content-paging-button"><i class="fas fa-angle-left"></i></button>
                        <div class="content-paging-info">
                            <p>{self.start}</p>
                            <p>{"    ...   "}</p>
                            <p>{self.start + self.take}</p>
                        </div>
                        <button onclick={ctx.link().callback(|_| ContentMessage::Next)} type="button" class="content-paging-button"><i class="fas fa-angle-right"></i></button>
                    </div>
                </div>
            </div>
        )
    }
}

impl ContentPage {
    fn get_content(&self) -> Html {
        self.content
            .iter()
            .map(|el| {
                html!(
                    <li class="content-element">
                        <div class="content-title">
                                <a target="_blank" href={el.link_url.clone()}>{el.title.clone()}</a>
                        </div>
                        <div class="content-desc"><SafeHtml html={match &el.description {
                                Some(desc) => desc.to_string().clone(),
                                None => "".to_string()
                        }}/></div>
                    </li>
                )
            })
            .collect::<Html>()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div: Element = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());
    let res = div.get_elements_by_tag_name("img");
    let mut i = 0;
    while let Some(element) = res.item(i) {
        element.set_class_name("image-fix");
        i += 1;
    }
    Html::VRef(div.into())
}
