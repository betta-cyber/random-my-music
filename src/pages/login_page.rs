use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{form_input::FormInput};
use crate::api::{user_api::login_api};


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
                let res = login_api(&form_json).await;
                match res {
                    Ok(_) => {
                        // navigator.push(&Route::Home);
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
                <FormInput label="Username" name="username" input_type="username" input_ref={username_input_ref} handle_onchange={handle_username_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
                <label for="password">{"Password"}</label>
                <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
                <button>{"Log In"}</button>
            </form>
        </div>
    }
}
