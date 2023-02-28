use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;
use crate::router::{switch, Route};
use crate::components::menu::Menu;
use crate::store::Store;
use crate::components::{
    alert::{AlertComponent, Props as AlertProps},
    spinner::Spinner,
};


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
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let is_page_loading = store.page_loading.clone();

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };
    html! {
        <main class="container">
            <Menu />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
                if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                     />
                }
                if is_page_loading {
                    <div class="pt-4 pl-2 top-[5.5rem] fixed">
                        <Spinner width={Some("1.5rem")} height={Some("1.5rem")} color="text-ct-yellow-600" />
                    </div>
                }
            </BrowserRouter>
        </main>
    }
}
