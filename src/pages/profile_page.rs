use crate::{
    api::user_api::{user_info_api, genres_api},
    // api::types::User,
    // router,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
    app::log,
    console_log
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::router::Route;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
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
                let genres = genres_api().await.unwrap();
                console_log!("{:#?}", genres);
            });
        },
        (),
    );

    html! {
    <>
      <section class="bg-ct-blue-600 min-h-screen pt-20">
        <div class="max-w-4xl mx-auto rounded-md h-[20rem] flex justify-center items-center">
          <div>
            <p class="text-5xl font-semibold">{"Profile Page"}</p>
            if let Some(user) = user {
              <div class="mt-8">
                <p class="mb-4">{format!("ID: {}", user.id)}</p>
                <p class="mb-4">{format!("Name: {}", user.username)}</p>
                <p class="mb-4">{format!("Email: {}", user.email)}</p>
                // <p class="mb-4">{format!("Role: {}", user.role)}</p>
              </div>
            }else {
              <p class="mb-4">{"Loading..."}</p>
            }
          </div>
        </div>
      </section>
    </>
    }
}
