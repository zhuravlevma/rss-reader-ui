use crate::pages::{
    home::HomePage, settings::SettingsPage, sign_in::SignInPage, sign_up::SignUpPage,
};
use crate::router::Route;
use yew::{html, Html};

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage />
        },
        Route::SignIn => html! {
            <SignInPage />
        },
        Route::SignUp => html! {
            <SignUpPage />
        },
        Route::Settings => html! {
            <SettingsPage />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
