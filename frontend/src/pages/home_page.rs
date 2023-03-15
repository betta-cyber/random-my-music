use yew::prelude::*;
use yewdux::prelude::*;
use web_sys::HtmlElement;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use uuid::Uuid;
use crate::api::user_api::today_album_api;
use crate::api::types::Album;
use crate::store::{Store, set_page_loading};
#[allow(unused_imports)]
use crate::{app::log, console_log};


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

    let detail_url = format!("/album/{}", album.id);

    let onerror = Callback::from(move | _e: Event | {
        // console_log!("{:#?}", e);
        // img_src = "https://randomyourmusic.fun/static/default.png".to_string();
    });

    let onload = Callback::from(move |e: Event| {
        if let Some(img) = e.target_dyn_into::<HtmlElement>() {
            img.toggle_attribute("hidden").unwrap();
        }

    });

    html! {
        <div class="album">
            <a href={detail_url} target="_blank">
                <i class="lazyload-img">
                    <img src={img_src} onerror={onerror} onload={onload} hidden=true />
                    // <img loading="lazy" src={img_src} onerror={onerror} onload={onload} hidden=true />
                </i>
            </a>
        </div>
    }
}


#[function_component(HomePage)]
pub fn home() -> Html {
    let (_, dispatch) = use_store::<Store>();
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

    let items = use_state(|| vec![]);
    {
        let items = items.clone();
        let store_dispatch = dispatch.clone();
        use_effect_with_deps(move |_| {
            let items = items.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = store_dispatch.clone();
                set_page_loading(true, dispatch.clone());
                match today_album_api(&client_id).await {
                    Ok(res) => {
                        items.set(res);
                        set_page_loading(false, dispatch);
                    }
                    Err(_) => {
                        set_page_loading(false, dispatch);
                    }
                }
            });
            || ()
        }, ());
    }

    let window = web_sys::window().expect("no global `window` exists");
    console_log!("{:#?}", window);

    html! {
        {
            items.iter().map(|album| {
                let album = album.clone();
                html!{ <AlbumCover album={album} /> }
            }).collect::<Html>()
        }
    }
}
