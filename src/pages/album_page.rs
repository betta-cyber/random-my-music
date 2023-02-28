use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::{Request, RequestCredentials};
use std::collections::HashMap;
// use crate::router::Route;


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
    let _navigator = use_navigator().unwrap();
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

    let _media_url = {
        if detail.media_url.contains_key("spotify") {
            let spotify_url = {
                let mut spotify = "";
                for (k, v) in detail.media_url.get("spotify").unwrap().as_object().unwrap() {
                    match v.get("default") {
                        Some(default) => {
                            if default.as_bool().unwrap() {
                                spotify = k;
                                break
                            }
                        }
                        None => {}
                    }
                }
                spotify
            };
            format!("https://open.spotify.com/album/{}", spotify_url)
        } else if detail.media_url.contains_key("soundcloud") {
            let soundcloud_url = {
                let mut soundcloud = "";
                for (_, v) in detail.media_url.get("soundcloud").unwrap().as_object().unwrap() {
                    let tmp = v.get("url").unwrap().as_str().unwrap();
                    soundcloud = tmp;
                    break
                }
                soundcloud
            };
            format!("https://{}", soundcloud_url)

        } else {
            format!("#")
        }
    };

    html! {
        <div>
            <div class="w-40 h-40">
                <img src={detail.cover.clone()} />
            </div>
            <div class="">
                <div class="lg:col-span-2 lg:border-r lg:border-gray-200 lg:pr-8">
                    <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{&detail.name}</h1>
                </div>
                <div class="lg:col-span-2 lg:border-r lg:border-gray-200 lg:pr-8">
                    <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{&detail.artist}</h1>
                </div>
                <div class="lg:col-span-2 lg:border-r lg:border-gray-200 lg:pr-8">
                    <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{&detail.descriptors}</h1>
                </div>
                <div class="lg:col-span-2 lg:border-r lg:border-gray-200 lg:pr-8">
                    <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{&detail.released}</h1>
                </div>
            </div>
        </div>
    }
}
