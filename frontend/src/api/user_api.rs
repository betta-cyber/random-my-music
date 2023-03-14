use serde::de::Deserialize;
use super::types::{ErrorResponse, JsonResponse, Album, AlbumDetail, Genre, User, AlbumLog};
use gloo_net::http::{Request, RequestCredentials};
#[allow(unused)]
use crate::{app::log, console_log};


static BASE_URL: &str = "/api/v1";


pub async fn make_request(url: &str, method: &str, data: Option<&str>) -> Result<String, String> {
    let req = match method {
        "GET" => {
            Request::get(&url)
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
        }
        "POST" => {
            Request::post(&url)
            .header("Content-Type", "application/json")
            .body(data.unwrap())
            .credentials(RequestCredentials::Include)
        }
        _ => {
            Request::get(&url)
            .credentials(RequestCredentials::Include)
            .body(data)
        }
    };

    let response = match req.send().await
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
    let res = response.text().await;
    match res {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to get response".to_string()),
    }
}

fn convert_result<'a, T: Deserialize<'a>>(
    input: &'a str,
) -> Result<T, String> {
    let result = serde_json::from_str::<T>(input).map_err(|e| {
        format!(
            "convert result failed, reason: {:?}; content: [{:?}]",
            e,
            input
        )
    })?;
    Ok(result)
}


pub async fn login_api(credentials: &str) -> Result<JsonResponse, String> {
    let url = format!("{}/login", BASE_URL);
    match make_request(&url, "POST", Some(credentials)).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn logout_api() -> Result<JsonResponse, String> {
    let url = format!("{}/logout", BASE_URL);
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn register_api(credentials: &str) -> Result<JsonResponse, String> {
    let url = format!("{}/register", BASE_URL);
    match make_request(&url, "POST", Some(credentials)).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn today_album_api(client_id: &str) -> Result<Vec<Album>, String> {
    let url = format!("{}/today?client_id={}", BASE_URL, client_id);
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<Vec<Album>>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}


pub async fn album_detail_api(album_id: &str) -> Result<AlbumDetail, String> {
    let url = format!("{}/album/{}", BASE_URL, album_id);
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<AlbumDetail>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn user_info_api() -> Result<User, String> {
    let url = format!("{}/user", BASE_URL);
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data).unwrap();
                    match serde_json::from_str::<User>(&serialized) {
                        Ok(user) => {Ok(user)},
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn genres_api() -> Result<Vec<Genre>, String> {
    let url = format!("{}/genres", BASE_URL);
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data.get("genres")).unwrap();
                    match serde_json::from_str::<Vec<Genre>>(&serialized) {
                        Ok(genres) => {Ok(genres)},
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}

pub async fn user_config_api(json: &str) -> Result<JsonResponse, String> {
    let url = format!("{}/user_config", BASE_URL);
    match make_request(&url, "POST", Some(json)).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}


pub async fn album_log_api(page: usize, page_size: usize) -> Result<Vec<AlbumLog>, String> {
    let url = format!("{}/user_album_log?page_size={}&page={}", BASE_URL, page_size, page);
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data.get("res")).unwrap();
                    match serde_json::from_str::<Vec<AlbumLog>>(&serialized) {
                        Ok(genres) => {Ok(genres)},
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e)
    }
}
