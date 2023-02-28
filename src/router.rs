use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    register_page::RegisterPage,
    login_page::SignInPage,
    home_page::HomePage,
    album_page::AlbumPage,
    about_page::AboutPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/album/*album_id")]
    Album { album_id: String },
    #[at("/register")]
    Register,
    #[at("/sign_in")]
    SignIn,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage />
        },
        Route::Album{ album_id } => html! {
            <AlbumPage album_id={album_id} />
        },
        Route::Register => html! {
            <RegisterPage />
        },
        Route::SignIn => html! {
            <SignInPage />
        },
        Route::About => html! {
            <AboutPage />
        },
        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}
