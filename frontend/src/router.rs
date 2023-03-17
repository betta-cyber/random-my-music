use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    about_page::AboutPage, album_page::AlbumPage, genre_page::GenrePage, history_page::HistoryPage,
    home_page::HomePage, login_page::SignInPage, profile_page::ProfilePage,
    register_page::RegisterPage, artist_page::ArtistPage,
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
    #[at("/profile")]
    Profile,
    #[at("/history")]
    History,
    #[at("/artist/*artist")]
    Artist { artist: String },
    #[at("/genre/*genre")]
    Genre { genre: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage />
        },
        Route::Album { album_id } => html! {
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
        Route::Profile => html! {
            <ProfilePage />
        },
        Route::History => html! {
            <HistoryPage />
        },
        Route::Artist { artist } => html! {
            <ArtistPage artist={artist} />
        },
        Route::Genre { genre } => html! {
            <GenrePage genre={genre} />
        },
        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}
