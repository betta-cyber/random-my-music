use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::api::{user_api::login_api};
use crate::store::{Store, set_show_alert, set_page_loading, set_auth_user};
use crate::router::Route;
use crate::api::types::User;
// use crate::{app::log, console_log};


#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate)]
struct LoginSchema {
    #[validate(
        length(min = 1, message = "Username is required"),
    )]
    username: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
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

    let (store, dispatch) = use_store::<Store>();
    let form = use_state(|| LoginSchema::default());
    let username_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let navigator = use_navigator().unwrap();
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let handle_username_input = get_input_callback("username", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "username" => data.username = value,
                "password" => data.password = value,
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

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_username_input_ref = username_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();
        let cloned_navigator = navigator.clone();
        let store_dispatch = dispatch.clone();

        Callback::from( move | event: SubmitEvent | {
            event.prevent_default();

            let form = cloned_form.clone();
            let username_input_ref = cloned_username_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();
            let navigator = cloned_navigator.clone();
            let dispatch = store_dispatch.clone();

            spawn_local(async move {
                let form_data = form.clone();
                let username_input = username_input_ref.cast::<HtmlInputElement>().unwrap();
                let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
                set_page_loading(true, dispatch.clone());

                let form_json = serde_json::to_string(&*form_data).unwrap();
                let res = login_api(&form_json).await;
                match res {
                    Ok(data) => {
                        let serialized = serde_json::to_string(&data.data).unwrap();
                        match serde_json::from_str::<User>(&serialized) {
                            Ok(user) => {
                                set_auth_user(Some(user), dispatch.clone());
                            }
                            Err(_) => {}
                        };
                        set_page_loading(false, dispatch);
                        navigator.push(&Route::Home);
                    }
                    Err(e) => {
                        set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                        username_input.set_value("");
                        password_input.set_value("");
                    }
                };
            });
        })
    };

    html! {
        <section class="bg-gray-600 min-h-screen grid place-items-center">
          <div class="w-full">
            <h1 class="text-4xl xl:text-6xl text-center font-[600] text-gray-800 mb-4">
              {"Welcome Back"}
            </h1>
            <h2 class="text-lg text-center mb-4 text-gray-700">
              {"Login to have access"}
            </h2>
              <form
                onsubmit={on_submit}
                class="max-w-md w-full mx-auto overflow-hidden shadow-2xl bg-gray-700 bg-opacity-50 rounded-2xl p-8 space-y-5"
              >
                <FormInput label="Username" name="username" input_type="username" input_ref={username_input_ref} handle_onchange={handle_username_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
                <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}/>

                <div class="text-right">
                  <a href="#" class="text-gray-800 font-medium hover:text-blue-400">
                    {"Forgot Password?"}
                  </a>
                </div>
                <LoadingButton
                  loading={store.page_loading}
                  text_color={Some("text-gray-800".to_string())}
                >
                  {"Login"}
                </LoadingButton>
                <span class="block">
                  <span class="text-gray-800"> {"Need an account?"} {" "} </span>
                  <Link<Route> to={Route::Register} classes="text-gray-800 font-medium hover:text-blue-400">{ "Sign Up Here" }</Link<Route>>
                </span>
              </form>
          </div>
        </section>
    }
}
