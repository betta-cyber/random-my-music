use serde::Deserialize;
use yew::prelude::*;
// use yew_router::prelude::*;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use gloo_net::http::{Request, RequestCredentials};
use uuid::Uuid;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Album {
    id: i32,
    name: String,
    artist: String,
    cover: String,
    media_url: HashMap<String, serde_json::Value>,
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub album: Album,
}



#[function_component]
fn AlbumCover(props: &Props) -> Html {
    let Props { album } = props;
    let img_src = {
        if album.cover.is_empty() {
            format!("/static/default.png")
        } else {
            format!("{}", album.cover)
        }
    };

    let _media_url = {
        if album.media_url.contains_key("spotify") {
            let spotify_url = {
                let mut spotify = "";
                for (k, v) in album.media_url.get("spotify").unwrap().as_object().unwrap() {
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
        } else if album.media_url.contains_key("soundcloud") {
            let soundcloud_url = {
                let mut soundcloud = "";
                for (_, v) in album.media_url.get("soundcloud").unwrap().as_object().unwrap() {
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
    let detail_url = format!("/album/{}", album.id);

    let on_error = Callback::from(move | _e: Event | {
        // console_log!("{:#?}", e);
        // img_src = "https://randomyourmusic.fun/static/default.png".to_string();
    });

    html! {
        <a href={detail_url} target="_blank">
            <div class="album">
                <img src={img_src} onerror={on_error} />
            </div>
        </a>
    }
}


#[function_component(HomePage)]
pub fn home() -> Html {
    let client_id: String = match LocalStorage::get("client_id") {
        Ok(client_id) => {
            client_id
        },
        Err(_) => {
            let client_id = Uuid::new_v4().to_string();
            LocalStorage::set("client_id", client_id.clone()).ok();
            client_id
        }
    };
    // let url = format!("https://rymbackend-production.up.railway.app/today?client_id={}", client_id);
    let url = format!("http://0.0.0.0:5001/today?client_id={}", client_id);

    let items = use_state(|| vec![]);
    {

        let items = items.clone();
        use_effect_with_deps(move |_| {
            let items = items.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get(&url)
                    .credentials(RequestCredentials::Include)
                    .send().await.unwrap();
                let data: Vec<Album> = res.json().await.unwrap();
                // console_log!("1. {:#?}", data);
                items.set(data);
            });
            || ()
        }, ());
    }

    html! {
        items.iter().map(|album| {
            let album = album.clone();
            html!{ <AlbumCover album={album} /> }
        }).collect::<Html>()
    }
}
