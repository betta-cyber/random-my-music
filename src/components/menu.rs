use web_sys::HtmlElement;
use yew::prelude::*;


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
