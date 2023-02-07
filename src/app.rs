use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use std::collections::HashMap;
use gloo_net::http::Request;
// use gloo_timers::callback::Timeout;


macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Album {
    name: String,
    artist: String,
    cover: String,
    media_url: HashMap<String, serde_json::Value>,
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// #[derive(Properties, PartialEq)]
// pub struct Props {
    // pub children: Children, // the field name `children` is important!
// }

#[derive(Properties, PartialEq)]
pub struct Props {
    pub album: Album,
}

#[function_component]
fn AlbumCover(props: &Props) -> Html {
    let Props { album } = props;
    let img_src = {
        if album.cover.contains("block") {
            format!("http://127.0.0.1:1420/public/default.png")
        } else {
            format!("{}", album.cover)
        }
    };

    let media_url = {
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
    html! {
        <a href={media_url} target="_blank">
            <div class="album">
                <img src={img_src} />
            </div>
        </a>
    }
}

#[function_component(App)]
pub fn app() -> Html {

    let url = "http://127.0.0.1:5000/today";

    let items = use_state(|| vec![]);
    {

        let items = items.clone();
        use_effect_with_deps(move |_| {
            let items = items.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get(url).send().await.unwrap();
                let data: Vec<Album> = res.json().await.unwrap();
                console_log!("1. {:#?}", data);

                items.set(data);
            });
            || ()
        }, ());
    }

    // let greet = {
        // let name = name.clone();
        // let greet_input_ref = greet_input_ref.clone();
        // Callback::from(move |_| {
            // name.set(greet_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
        // })
    // };


    html! {
        <main class="container">
            {
                items.iter().map(|album| {
                    let album = album.clone();
                    html!{ <AlbumCover album={album} /> }
                }).collect::<Html>()
            }
        </main>
    }
}
