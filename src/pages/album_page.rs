use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::{Request, RequestCredentials};
use std::collections::HashMap;
use crate::router::Route;


#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct AlbumDetail {
    id: i32,
    name: String,
    artist: String,
    cover: String,
    media_url: HashMap<String, serde_json::Value>,
    descriptors: String,
    released: String,
}

#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub album_id: String,
}

#[function_component(AlbumPage)]
pub fn album(props: &DetailProps) -> Html {
    let navigator = use_navigator().unwrap();
    let DetailProps { album_id } = props;
    // let url = format!("https://rymbackend-production.up.railway.app/album/{}", album_id);
    let url = format!("http://0.0.0.0:5001/album/{}", album_id);
    let detail = use_state(|| AlbumDetail::default());
    {
        let detail = detail.clone();
        use_effect_with_deps(move |_| {
            let detail = detail.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get(&url)
                    .credentials(RequestCredentials::Include)
                    .send().await.unwrap();
                let data: AlbumDetail = res.json().await.unwrap();
                detail.set(data);
            });
            || ()
        }, ());
    }

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <button {onclick}>{ "Go Home" }</button>
            <div class="detal">
            {detail.id}
            <span>{&detail.cover}</span>
            <span>{&detail.artist}</span>
            <span>{&detail.descriptors}</span>
            <span>{&detail.released}</span>
            </div>
        </div>
    }
}
