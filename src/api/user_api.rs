use super::types::{ErrorResponse, User, LoginResponse, UserResponse};
use gloo_net::http::Request;

pub async fn login(credentials: &str) -> Result<LoginResponse, String> {
    let response = match Request::post("http://0.0.0.0:5001/login")
        .header("Content-Type", "application/json")
        // .credentials(http::RequestCredentials::Include)
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
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<LoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

// let res = Request::get(&url).send().await.unwrap();
                // let data: Vec<Album> = res.json().await.unwrap();
                // console_log!("1. {:#?}", data);
                // items.set(data);

