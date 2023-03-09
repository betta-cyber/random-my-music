use super::types::{ErrorResponse, JsonResponse, Album, AlbumDetail, Genre, User};
use gloo_net::http::{Request, RequestCredentials};
#[allow(unused)]
use crate::{app::log, console_log};


static BASE_URL: &str = "/api/v1";
// static BASE_URL: &str = "http://0.0.0.0:5001";
// static BASE_URL: &str = "https://rymbackend-production.up.railway.app";

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


pub async fn logout_api() -> Result<JsonResponse, String> {
    let response = match Request::get(&format!("{}/logout", BASE_URL))
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
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

pub async fn today_album_api(client_id: &str) -> Result<Vec<Album>, String> {
    let url = format!("{}/today?client_id={}", BASE_URL, client_id);
    let response = match Request::get(&url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
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

    let res_data = response.json::<Vec<Album>>().await;
    // let res_json = response.json::<JsonResponse>().await;
    match res_data {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}


pub async fn album_detail_api(album_id: &str) -> Result<AlbumDetail, String> {
    let url = format!("{}/album/{}", BASE_URL, album_id);
    let response = match Request::get(&url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
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

    let res_data = response.json::<AlbumDetail>().await;
    match res_data {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}


pub async fn user_info_api() -> Result<User, String> {
    let url = format!("{}/user", BASE_URL);
    let response = match Request::get(&url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
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

    let res_data = response.json::<JsonResponse>().await;
    match res_data {
        Ok(data) => {
            let serialized = serde_json::to_string(&data.data).unwrap();
            match serde_json::from_str::<User>(&serialized) {
                Ok(user) => {
                    Ok(user)
                },
                Err(_) => Err("Failed to parse response".to_string()),
            }
        },
        Err(_) => Err("Failed to parse response".to_string())
    }
}

pub async fn genres_api() -> Result<Vec<Genre>, String> {
    let url = format!("{}/genres", BASE_URL);
    let response = match Request::get(&url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
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

    let res_data = response.json::<JsonResponse>().await;
    match res_data {
        Ok(data) => {
            let serialized = serde_json::to_string(&data.data.get("genres")).unwrap();
            match serde_json::from_str::<Vec<Genre>>(&serialized) {
                Ok(genres) => {
                    Ok(genres)
                },
                Err(_) => Err("Failed to parse response".to_string()),
            }
        },
        Err(_) => Err("Failed to parse response".to_string())
    }
}


pub async fn user_config_api(json: &str) -> Result<JsonResponse, String> {
    let url = format!("{}/user_config", BASE_URL);
    let response = match Request::post(&url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(json)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            console_log!("{:#?}", e);
            return Err("Failed to make request".to_string())
        }
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.msg);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_data = response.json::<JsonResponse>().await;
    console_log!("{:#?}", res_data);
    match res_data {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string())
    }
}
