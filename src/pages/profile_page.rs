use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::{
    api::user_api::{user_info_api, genres_api, user_config_api},
    router::Route,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
    app::log,
    console_log,
    components::form_input::FormInput,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;


#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate)]
struct ProfileSchema {
    genres: String,
    #[validate(length(min = 1, max = 4, message = "Fresh time require 1 - 3600 min"))]
    fresh_time: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<ProfileSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "fresh_time" => data.fresh_time = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}


#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    let fresh_time_input_ref = NodeRef::default();
    let genres = use_state(|| vec![]);
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let form = use_state(|| ProfileSchema::default());
    let handle_fresh_time_input = get_input_callback("fresh_time", form.clone());

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "fresh_time" => data.fresh_time = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };


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
        let fresh_time_input_ref = fresh_time_input_ref.clone();
        let store_dispatch = dispatch.clone();
        let user = user.clone();
        Callback::from( move | _: MouseEvent | {
            let mut user_genre: Vec<String> = vec![];
            let window = web_sys::window().expect("no global `window` exists");
            let genres = window.document().unwrap().get_elements_by_class_name("genre");
            for i in 0..genres.length() {
                if let Ok(g) = genres.item(i).unwrap().dyn_into::<HtmlInputElement>() {
                    if g.checked() {
                        user_genre.push(g.value());
                    }
                }
            }
            let genre_str = user_genre.join(",");
            let fresh_time_input_ref = fresh_time_input_ref.clone();
            let fresh_time_input = fresh_time_input_ref.cast::<HtmlInputElement>().unwrap().value().parse::<i32>().unwrap();
            let form = ProfileSchema { genres: genre_str.clone(), fresh_time: fresh_time_input.to_string()};
            let dispatch = store_dispatch.clone();
            let user = user.clone();

            spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let mut user = user.clone();
                let json = serde_json::to_string(&form).unwrap();
                let res = user_config_api(&json).await;
                match res {
                    Ok(data) => {
                        // update user store
                        user.as_mut().unwrap().genre_data = genre_str;
                        user.as_mut().unwrap().fresh_time = fresh_time_input;
                        set_auth_user(user, dispatch.clone());
                        set_page_loading(false, dispatch.clone());
                        set_show_alert(data.msg, dispatch);
                    }
                    Err(e) => {
                        set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            });
        })
    };


    html! {
    <>
      <section class="bg-ct-blue-600 min-h-screen pt-20">
        <div class="max-w-4xl mx-auto rounded-md">
          <div class="max-w-md w-full mx-auto overflow-hidden shadow-2xl bg-gray-700 bg-opacity-50 rounded-2xl p-8 space-y-5">
            <p class="text-5xl font-semibold">{"Profile Page"}</p>
            if let Some(user) = user {
                <div class="mt-8">
                    <p class="mb-4">{format!("Name: {}", user.username)}</p>
                    <p class="mb-4">{format!("Email: {}", user.email)}</p>
                    // <p class="mb-4">{format!("Role: {}", user.role)}</p>
                </div>
                <div>
                <p class="mb-4">{format!("Genres:")}</p>
                {
                    genres.iter().map(|g| {
                    html! {
                        <div class="float-left m-2">
                            <input class="genre" type="checkbox" checked={
                                if user.genre_data.contains(&g.key_name) {
                                    true
                                } else {
                                    false
                                }
                            } value={g.key_name.clone()}/>
                            <label for={g.key_name.clone()} >{ &g.name }</label>
                        </div>
                    }
                    }).collect::<Html>()
                }
                <div class="float-left w-full">
                    <FormInput label="Fresh Time: [3-3600] mins" name="fresh_time" input_type="" input_ref={fresh_time_input_ref} handle_onchange={handle_fresh_time_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
                </div>
                <button onclick={on_submit}>{"Update"}</button>
                // <LoadingButton
                    // loading={store.page_loading}
                    // text_color={Some("text-gray-800".to_string())}
                // >
                  // {"Update"}
                // </LoadingButton>
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
