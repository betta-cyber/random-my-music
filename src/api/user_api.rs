use super::types::{ErrorResponse, JsonResponse};
use gloo_net::http::{Request, RequestCredentials};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// macro_rules! console_log {
    // // Note that this is using the `log` function imported above during
    // // `bare_bones`
    // ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

static BASE_URL: &str = "http://0.0.0.0:5001";

pub async fn login_api(credentials: &str) -> Result<JsonResponse, String> {
    let response = match Request::post(&format!("{}/login", BASE_URL))
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.msg);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<JsonResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn register_api(credentials: &str) -> Result<JsonResponse, String> {
    let response = match Request::post(&format!("{}/register", BASE_URL))
        .header("Content-Type", "application/json")
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.msg);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<JsonResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}


