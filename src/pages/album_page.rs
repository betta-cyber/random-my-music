use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::{Request, RequestCredentials};
use std::collections::HashMap;
use crate::components::media_link::MediaLink;


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
        <div class="block bg-blue-800 w-auto h-screen">
            <div class="absolute inset-y-0 left-0 w-2/6" id="container_left">
                <div class="m-4">
                    <img class="w-full" src={detail.cover.clone()} />
                    <div class="media-link h-40 object-center flex">
                        <MediaLink media_data={detail.media_url.clone()}></MediaLink>
                    </div>
                </div>
            </div>
            <div class="text-left absolute inset-y-0 right-0 w-4/6" id="container_right">
                <div class="width-full album_info_outer m-4">
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <h1 class="col-span-3 m-2 text-left float-left text-4xl font-bold tracking-tight text-gray-900 sm:text-4xl">{&detail.name}</h1>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Artist"}</span>
                        <span class="col-span-2 m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.artist}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Released"}</span>
                        <span class="col-span-2 m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.released}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8 ">
                        <span class="col-span-1 m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Descriptors"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.descriptors}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
