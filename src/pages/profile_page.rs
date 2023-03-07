use crate::{
    api::user_api::{user_info_api, genres_api, user_genres_api},
    router::Route,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
    app::log,
    console_log,
    components::loading_button::LoadingButton,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;


#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    let genres = use_state(|| vec![]);

    {
        let genres = genres.clone();
        let dispatch = dispatch.clone();
        use_effect_with_deps(move |_| {
            let dispatch = dispatch.clone();
            let genres = genres.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let response = user_info_api().await;
                match response {
                    Ok(user) => {
                        set_page_loading(false, dispatch.clone());
                        set_auth_user(Some(user), dispatch);
                    }
                    Err(e) => {
                        set_page_loading(false, dispatch.clone());
                        if e.contains("not logged") {
                            navigator.push(&Route::SignIn);
                        }
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
                if let Ok(data) = genres_api().await {
                    genres.set(data);
                }
            });
        },
        ());
    }

    let on_submit = {
        // let cloned_form = form.clone();
        Callback::from( move | _: SubmitEvent | {
            let window = web_sys::window().expect("no global `window` exists");
            let genres = window.document().unwrap().get_elements_by_class_name("genre");
            let mut user_genre: Vec<String> = vec![];
            for i in 0..genres.length() {
                if let Ok(g) = genres.item(i).unwrap().dyn_into::<HtmlInputElement>() {
                    if g.checked() {
                        user_genre.push(g.value());
                    }
                }
            }
            console_log!("{:#?}", user_genre);
            spawn_local(async move {
                let json = serde_json::to_string(&*user_genre).unwrap();
                let res = user_genres_api(&json).await;
                console_log!("{:#?}", res);
            });
        })
    };


    html! {
    <>
      <section class="bg-ct-blue-600 min-h-screen pt-20">
        <div class="max-w-4xl mx-auto rounded-md">
          <div>
            <p class="text-5xl font-semibold">{"Profile Page"}</p>
            if let Some(user) = user {
              <div class="mt-8">
                <p class="mb-4">{format!("Name: {}", user.username)}</p>
                <p class="mb-4">{format!("Email: {}", user.email)}</p>
                // <p class="mb-4">{format!("Role: {}", user.role)}</p>
              </div>
                <form
                    onsubmit={on_submit}
                    class="max-w-md w-full mx-auto overflow-hidden shadow-2xl bg-gray-700 bg-opacity-50 rounded-2xl p-8 space-y-5"
                >
                {
                    genres.iter().map(|g| {
                    html! {
                        <div>
                            <input class="genre" type="checkbox" checked=false value={g.key_name.clone()}/>
                            <label for={g.key_name.clone()} >{ &g.name }</label>
                        </div>
                    }
                    }).collect::<Html>()
                }
                <LoadingButton
                  loading={store.page_loading}
                  text_color={Some("text-gray-800".to_string())}
                >
                  {"Update"}
                </LoadingButton>
                </form>
            }else {
              <p class="mb-4">{"Loading..."}</p>
            }
          </div>
        </div>
      </section>
    </>
    }
}
