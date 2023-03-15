use yew::prelude::*;
use yewdux::prelude::*;
use web_sys::HtmlElement;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
// use gloo_timers::callback::Timeout;
use uuid::Uuid;
use crate::api::user_api::today_album_api;
// use crate::api::types::Album;
use crate::store::{Store, set_page_loading};
#[allow(unused_imports)]
use crate::{app::log, console_log};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    pub cover: String,
}



#[function_component]
fn AlbumCover(props: &Props) -> Html {
    let id = props.id;
    let cover = props.cover.clone();

    let img_src = {
        if cover.is_empty() {
            format!("/static/default.png")
        } else {
            format!("{}-cover", cover)
        }
    };

    let detail_url = format!("/album/{}", id);
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
                    // <img loading="lazy" src={img_src} onerror={onerror} />
                </i>
            </a>
        </div>
    }
}


#[function_component(HomePage)]
pub fn home() -> Html {
    let (_, dispatch) = use_store::<Store>();
    let page = use_state(||1);
    let items = use_state(|| vec![]);
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

    {
        let items = items.clone();
        let client_id = client_id.clone();
        let page = page.clone();
        let store_dispatch = dispatch.clone();
        use_effect_with_deps(move |_| {
            let items = items.clone();
            let client_id = client_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = store_dispatch.clone();
                set_page_loading(true, dispatch.clone());
                match today_album_api(&client_id, *page, 40).await {
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


    // let items_clone = items.clone();
    // let waiting = use_state(||false);
    // let onscroll = Callback::from(move|e: WheelEvent| {
        // let client_id = client_id.clone();
        // let page = page.clone();
        // let items = items_clone.clone();
        // // let dispatch = dispatch.clone();
        // if let Some(_) = e.target_dyn_into::<HtmlElement>() {
            // if *waiting {
                // return {}
            // }
            // let window = web_sys::window().expect("no global `window` exists");
            // // scroll_top为滚动条在Y轴上的滚动距离。
            // let scroll_y = window.scroll_y().unwrap();
            // //client_height为内容可视区域的高度。
            // let client_height = window.inner_height().unwrap().as_f64().unwrap();
            // // scroll_height为内容可视区域的高度加上溢出（滚动）的距离。
            // let scroll_height = window.document().unwrap().body().unwrap().scroll_height() as f64;
            // if scroll_height - (scroll_y + client_height) < scroll_height / 8.0 {
                // wasm_bindgen_futures::spawn_local(async move {
                    // // set_page_loading(true, dispatch.clone());
                    // page.set(*page+1);
                    // match today_album_api(&client_id, *page+1, 40).await {
                        // Ok(res) => {
                            // let mut r = items.deref().clone();
                            // r.extend(res);
                            // items.set(r);
                            // // set_page_loading(false, dispatch);
                        // }
                        // Err(_) => {}
                    // }
                // });
            // }
            // waiting.set(true);
            // let waiting = waiting.clone();
            // let _ = Timeout::new(300, move || {
                // let waiting = waiting.clone();
                // waiting.set(false);
            // }).forget();
        // }
    // });


    html! {
        // <div onwheel={onscroll} >
        {
            items.iter().map(move |album| {
                html!{ <AlbumCover id={album.id} cover={album.cover.clone()}/> }
            }).collect::<Html>()
        }
        // </div>
    }
}
