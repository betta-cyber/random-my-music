use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user_api::register_api;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::Route;
use crate::store::{set_show_alert, Store};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct RegisterUserSchema {
    #[validate(length(min = 1, message = "Username is required"))]
    username: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
    #[validate(
        length(min = 1, message = "Please confirm your password"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    password_confirm: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<RegisterUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "username" => data.username = value,
            "email" => data.email = value,
            "password" => data.password = value,
            "password_confirm" => data.password_confirm = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form = use_state(RegisterUserSchema::default);
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let username_input_ref = NodeRef::default();
    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let password_confirm_input_ref = NodeRef::default();

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "email" => data.email = value,
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

    let handle_username_input = get_input_callback("username", form.clone());
    let handle_email_input = get_input_callback("email", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());
    let handle_password_confirm_input = get_input_callback("password_confirm", form.clone());

    let on_submit = {
        let cloned_form = form;
        let cloned_validation_errors = validation_errors.clone();
        let cloned_navigator = navigator;
        let cloned_dispatch = dispatch;

        let cloned_username_input_ref = username_input_ref.clone();
        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();
        let cloned_password_confirm_input_ref = password_confirm_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();
            let dispatch = cloned_dispatch.clone();

            let username_input_ref = cloned_username_input_ref.clone();
            let email_input_ref = cloned_email_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();
            let password_confirm_input_ref = cloned_password_confirm_input_ref.clone();

            event.prevent_default();
            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        let form_json = serde_json::to_string(&form_data).unwrap();
                        // set_page_loading(true, dispatch.clone());

                        let username_input = username_input_ref.cast::<HtmlInputElement>().unwrap();
                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_confirm_input = password_confirm_input_ref
                            .cast::<HtmlInputElement>()
                            .unwrap();

                        username_input.set_value("");
                        email_input.set_value("");
                        password_input.set_value("");
                        password_confirm_input.set_value("");

                        let res = register_api(&form_json).await;
                        match res {
                            Ok(_) => {
                                // set_page_loading(false, dispatch.clone());
                                set_show_alert(
                                    "Account registered successfully".to_string(),
                                    dispatch,
                                );
                                navigator.push(&Route::SignIn);
                            }
                            Err(e) => {
                                // set_page_loading(false, dispatch.clone());
                                set_show_alert(e, dispatch);
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
    <section class="py-8 bg-gray-600 min-h-screen grid place-items-center">
      <div class="w-full">
        <h1 class="text-4xl xl:text-6xl text-center font-[600] text-gray-800 mb-4">
         {" Welcome to Random Your Music!"}
        </h1>
        <h2 class="text-lg text-center mb-4 text-gray-700">
          {"Sign Up To Get Started!"}
        </h2>
          <form
            onsubmit={on_submit}
            class="max-w-md w-10/12 mx-auto overflow-hidden shadow-2xl bg-gray-700 bg-opacity-50 rounded-2xl p-8 space-y-5"
          >
            <FormInput label="Userame" name="name" input_ref={username_input_ref} handle_onchange={handle_username_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Email" name="email" input_type="email" input_ref={email_input_ref} handle_onchange={handle_email_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput
              label="Confirm Password"
              name="password_confirm"
              input_type="password"
              input_ref={password_confirm_input_ref}
              handle_onchange={handle_password_confirm_input}
              errors={&*validation_errors}
              handle_on_input_blur={validate_input_on_blur.clone()}
            />
            <span class="block text-gray-800">
              {"Already have an account?"} {" "}
            <Link<Route> to={Route::SignIn} classes="text-gray-800 font-medium hover:text-blue-400">{"Login Here"}</Link<Route>>
            </span>
            <LoadingButton
              loading={store.page_loading}
              text_color={Some("text-gray-800".to_string())}
            >
             {" Sign Up"}
            </LoadingButton>
          </form>
      </div>
    </section>
    }
}
