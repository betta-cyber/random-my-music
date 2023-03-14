use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::*;
// use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api::user_api::logout_api;
use crate::store::{Store, set_show_alert, set_page_loading, set_auth_user};
// use crate::router::Route;
#[allow(unused_imports)]
use crate::{app::log, console_log};


#[function_component]
pub fn Menu() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    // let navigator = use_navigator().unwrap();

    let onclick = Callback::from(|e: MouseEvent| {
        if let Some(_) = e.target_dyn_into::<HtmlElement>() {
            let window = web_sys::window().expect("no global `window` exists");
            let m = window.document().unwrap().get_elements_by_class_name("menu").item(0).unwrap();

            if m.class_name() == "menu" {
                m.set_class_name("menu change");
            } else if m.class_name() == "menu change" {
                m.set_class_name("menu");
            }

            let menu = window.document().unwrap().get_elements_by_class_name("menu-item").item(0).unwrap();
            if menu.class_name() == "menu-item" {
                menu.set_class_name("menu-item menu-display");
            } else if menu.class_name() == "menu-item menu-display" {
                menu.set_class_name("menu-item");
            }
        }
    });

    let logout = Callback::from( move |_: MouseEvent| {
        // let navigator = navigator.clone();
        let dispatch = dispatch.clone();
        spawn_local(async move {
            // let navigator = navigator.clone();
            set_page_loading(true, dispatch.clone());
            let res = logout_api().await;
            match res {
                Ok(data) => {
                    set_auth_user(None, dispatch.clone());
                    set_page_loading(false, dispatch.clone());
                    set_show_alert(data.msg.to_string(), dispatch);
                    // navigator.push(&Route::Home);
                }
                Err(e) => {
                    set_page_loading(false, dispatch.clone());
                    set_show_alert(e.to_string(), dispatch);
                }
            };
        });
    });

    html! {
        <>
        <div class="menu" onclick={onclick.clone()}>
            <div class="bar1"></div>
            <div class="bar2"></div>
            <div class="bar3"></div>
        </div> <div class="menu-item">
            <div class="">
                <ul>
                    <li><a href="/" >{"Home"}</a></li>
                    if let Some(user) = user {
                        <li><a href="/profile" onclick={onclick.clone()}>{ user.username }</a></li>
                        <li><a href="/history" onclick={onclick.clone()}>{ "History" }</a></li>
                        <li><a onclick={logout}>{"Sign out"}</a></li>
                    } else {
                        <li><a href="/sign_in" onclick={onclick.clone()}>{"Sign in"}</a></li>
                        <li><a href="/register" onclick={onclick.clone()}>{"Sign up"}</a></li>
                    }
                    <li><a href="/about" onclick={onclick}>{"About"}</a></li>
                </ul>
            </div>
        </div>
        </>
    }
}
