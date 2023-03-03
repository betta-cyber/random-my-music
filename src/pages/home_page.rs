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
            format!("{}-cover", album.cover)
        }
    };

    let detail_url = format!("/#/album/{}", album.id);

    let on_error = Callback::from(move | _e: Event | {
        // console_log!("{:#?}", e);
        // img_src = "https://randomyourmusic.fun/static/default.png".to_string();
    });

    html! {
        <div class="album">
            <a href={detail_url} target="_blank">
                <img loading="lazy" src={img_src} onerror={on_error} />
            </a>
        </div>
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
