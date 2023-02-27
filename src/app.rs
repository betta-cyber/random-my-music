use std::ops::Deref;
use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::{function_component, html, Html, Properties};
use std::collections::HashMap;
use gloo_net::http::Request;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use uuid::Uuid;
use web_sys::{HtmlElement, MouseEvent, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use crate::components::{form_input::FormInput};
use crate::api::{user_api::login};

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Album {
    id: i32,
    name: String,
    artist: String,
    cover: String,
    media_url: HashMap<String, serde_json::Value>,
}


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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub album: Album,
}

#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub album_id: String,
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

    let on_error = Callback::from(move | e: Event | {
        console_log!("{:#?}", e);
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


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage />
        },
        Route::Album{ album_id } => html! {
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
        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}


#[function_component]
pub fn Menu() -> Html {

    let onclick = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let class_name = target.class_name();
            if class_name == "menu" {
                target.set_class_name("menu change");
            } else if class_name == "menu change" {
                target.set_class_name("menu");
            }

            let window = web_sys::window().expect("no global `window` exists");
            let menu = window.document().unwrap().get_elements_by_class_name("menu-item").item(0).unwrap();
            if menu.class_name() == "menu-item" {
                menu.set_class_name("menu-item menu-display");
            } else if menu.class_name() == "menu-item menu-display" {
                menu.set_class_name("menu-item");
            }
        }
    });

    html! {
        <>
        <div class="menu" onclick={onclick}>
            <div class="bar1"></div>
            <div class="bar2"></div>
            <div class="bar3"></div>
        </div>
        <div class="menu-item">
            <div class="">
                <ul>
                  <li><a href="/">{"Home"}</a></li>
                  <li><a href="/register">{"Sign up"}</a></li>
                  <li><a href="/sign_in">{"Sign in"}</a></li>
                  <li><a href="/about">{"About"}</a></li>
                </ul>
            </div>
        </div>
        </>
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <Menu />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </main>
    }
}


#[function_component(HomePage)]
fn home() -> Html {
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
                let res = Request::get(&url).send().await.unwrap();
                let data: Vec<Album> = res.json().await.unwrap();
                console_log!("1. {:#?}", data);
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

#[function_component(AlbumPage)]
fn album(props: &DetailProps) -> Html {
    let navigator = use_navigator().unwrap();
    let DetailProps { album_id } = props;
    let url = format!("https://rymbackend-production.up.railway.app/album/{}", album_id);
    let detail = use_state(|| AlbumDetail::default());
    {
        let detail = detail.clone();
        use_effect_with_deps(move |_| {
            let detail = detail.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get(&url).send().await.unwrap();
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

#[function_component(RegisterPage)]
pub fn register() -> Html {
    html! {
        <div class="about">
        </div>
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginSchema {
    // #[validate(
        // length(min = 1, message = "Email is required"),
        // email(message = "Email is invalid")
    // )]
    username: String,
    // #[validate(
        // length(min = 1, message = "Password is required"),
        // length(min = 6, message = "Password must be at least 6 characters")
    // )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "username" => data.username = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(SignInPage)]
pub fn sign_in() -> Html {

    let form = use_state(|| LoginSchema::default());
    let username_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let navigator = use_navigator().unwrap();

    let handle_username_input = get_input_callback("username", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_username_input_ref = username_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();
        let cloned_navigator = navigator.clone();

        Callback::from( move | event: SubmitEvent | {
            event.prevent_default();

            let form = cloned_form.clone();
            let username_input_ref = cloned_username_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();
            let navigator = cloned_navigator.clone();

            spawn_local(async move {
                let form_data = form.clone();
                let username_input = username_input_ref.cast::<HtmlInputElement>().unwrap();
                let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                username_input.set_value("");
                password_input.set_value("");
                let form_json = serde_json::to_string(&*form_data).unwrap();
                let res = login(&form_json).await;
                match res {
                    Ok(_) => {
                        navigator.push(&Route::Home);
                    }
                    Err(_) => {
                        // set_show_alert(e.to_string(), dispatch);
                    }
                };
            });
        })
    };

    html! {
        <div class="sign-in">
            <form onsubmit={on_submit}>
                <h3>{"Login Here"}</h3>
                <label for="username">{"Username"}</label>
                <FormInput label="Username" name="username" input_type="username" input_ref={username_input_ref} handle_onchange={handle_username_input}/>
                <label for="password">{"Password"}</label>
                <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input}/>
                <button>{"Log In"}</button>
            </form>
        </div>
    }
}

#[function_component(AboutPage)]
pub fn about() -> Html {
    html! {
        <div class="about">
            {"也许我们不能通过一本书的封面来判断书的质量，但当我们谈论到音乐时，专辑封面有时候会扮演一个重要的角色，他们是专辑的视觉表达。自从黑胶唱片诞生以来，专辑封面经过长时间的发展，已经从简单的个性表达逐步演变成了复杂的艺术作品。我喜欢看它们，也喜欢谈论它们，它们为音乐赋予了视觉表现力。一张专辑的封面背后往往有许多关于音乐的有趣故事，所以我设计创作了这个网站。致力于从封面视觉出发，基于音乐流派，让用户探索发现更多有趣的音乐。"}
        </div>
    }
}
