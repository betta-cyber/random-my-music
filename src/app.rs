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
        delay_ms: 500,
    };
    html! {
        <main class="container max-w-full">
            <Menu />
            <BrowserRouter>
                if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                     />
                }
                if is_page_loading {
                    <div class="loading">
                        <Spinner width={Some("1.8rem")} height={Some("1.8rem")} color="text-ct-red-600" />
                    </div>
                }
                <div hidden={is_page_loading}>
                    <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
                </div>
            </BrowserRouter>
        </main>
    }
}
