use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

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
    pub aid: i32,
}

// #[function_component]
// fn HelloWorld(props: &Props) -> Html {
    // html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
// }

#[function_component]
fn AlbumCover(props: &Props) -> Html {
    let Props { aid } = props;
    let img_src = format!("public/albums/{}.jpg", aid);
    html! {
        <img src={img_src} width="250" height="250" />
    }
}

#[function_component(App)]
pub fn app() -> Html {

    // let img: Html = html! {
        // <img src="public/albums/1.jpg" alt="Girl in a jacket" width="150" height="150" />
    // };
    // let greet_input_ref = use_node_ref();

    // let name = use_state(|| String::new());

    // let greet_msg = use_state(|| String::new());
    // {
        // let greet_msg = greet_msg.clone();
        // let name = name.clone();
        // let name2 = name.clone();
        // use_effect_with_deps(
            // move |_| {
                // spawn_local(async move {
                    // if name.is_empty() {
                        // return;
                    // }

                    // // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    // let new_msg = invoke(
                        // "greet",
                        // to_value(&GreetArgs { name: &*name }).unwrap(),
                    // )
                    // .await;
                    // log(&new_msg.as_string().unwrap());
                    // greet_msg.set(new_msg.as_string().unwrap());
                // });

                // || {}
            // },
            // name2,
        // );
    // }

    // let greet = {
        // let name = name.clone();
        // let greet_input_ref = greet_input_ref.clone();
        // Callback::from(move |_| {
            // name.set(greet_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
        // })
    // };


    let items = (1..=10).collect::<Vec<_>>();

    html! {
        <main class="container">
            {
                items.into_iter().map(|name| {
                    html!{ <AlbumCover aid={name} /> }
                    // html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
                }).collect::<Html>()
            }
        </main>
    }
}
