use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{switch, Route};
use crate::components::menu::Menu;
// use web_sys::console::log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
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
