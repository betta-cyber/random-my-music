use super::types::{
    Album, AlbumDetail, AlbumLogData, ErrorResponse, Genre, ChartData, JsonResponse, User,
};
#[allow(unused)]
use crate::{app::log, console_log};
use gloo_net::http::{Request, RequestCredentials};
use serde::de::Deserialize;

static BASE_URL: &str = "/api/v1";

pub async fn make_request(url: &str, method: &str, data: Option<&str>) -> Result<String, String> {
    let req = match method {
        "GET" => Request::get(url)
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include),
        "POST" => Request::post(url)
            .header("Content-Type", "application/json")
            .body(data.unwrap())
            .credentials(RequestCredentials::Include),
        _ => Request::get(url)
            .credentials(RequestCredentials::Include)
            .body(data),
    };

    let response = match req.send().await {
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

fn convert_result<'a, T: Deserialize<'a>>(input: &'a str) -> Result<T, String> {
    let result = serde_json::from_str::<T>(input).map_err(|e| {
        format!(
            "convert result failed, reason: {e:?}; content: [{input:?}]"
        )
    })?;
    Ok(result)
}

pub async fn login_api(credentials: &str) -> Result<JsonResponse, String> {
    let url = format!("{BASE_URL}/login");
    match make_request(&url, "POST", Some(credentials)).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn logout_api() -> Result<JsonResponse, String> {
    let url = format!("{BASE_URL}/logout");
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn register_api(credentials: &str) -> Result<JsonResponse, String> {
    let url = format!("{BASE_URL}/register");
    match make_request(&url, "POST", Some(credentials)).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn today_album_api(
    client_id: &str,
    page: i32,
    page_size: i32,
) -> Result<Vec<Album>, String> {
    let url = format!(
        "{BASE_URL}/today?client_id={client_id}&page={page}&page_size={page_size}"
    );
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<Vec<Album>>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn album_detail_api(album_id: &str) -> Result<AlbumDetail, String> {
    let url = format!("{BASE_URL}/album/{album_id}");
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<AlbumDetail>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn user_info_api() -> Result<User, String> {
    let url = format!("{BASE_URL}/user");
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data).unwrap();
                    match serde_json::from_str::<User>(&serialized) {
                        Ok(user) => Ok(user),
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn genres_api() -> Result<Vec<Genre>, String> {
    let url = format!("{BASE_URL}/genres");
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data.get("genres")).unwrap();
                    match serde_json::from_str::<Vec<Genre>>(&serialized) {
                        Ok(genres) => Ok(genres),
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn user_config_api(json: &str) -> Result<JsonResponse, String> {
    let url = format!("{BASE_URL}/user_config");
    match make_request(&url, "POST", Some(json)).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn album_log_api(page: u32, page_size: u32) -> Result<AlbumLogData, String> {
    let url = format!(
        "{BASE_URL}/user_album_log?page_size={page_size}&page={page}"
    );
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data).unwrap();
                    match serde_json::from_str::<AlbumLogData>(&serialized) {
                        Ok(data) => Ok(data),
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn genre_album_api(genre: &str, page: u32, page_size: u32) -> Result<ChartData, String> {
    let url = format!(
        "{BASE_URL}/genre/{genre}?page_size={page_size}&page={page}"
    );
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data).unwrap();
                    match serde_json::from_str::<ChartData>(&serialized) {
                        Ok(genres) => Ok(genres),
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}


pub async fn artist_album_api(artist: &str, page: u32, page_size: u32) -> Result<ChartData, String> {
    let url = format!(
        "{BASE_URL}/artist/{artist}?page_size={page_size}&page={page}"
    );
    match make_request(&url, "GET", None).await {
        Ok(response) => {
            let res = convert_result::<JsonResponse>(&response);
            match res {
                Ok(data) => {
                    let serialized = serde_json::to_string(&data.data).unwrap();
                    match serde_json::from_str::<ChartData>(&serialized) {
                        Ok(genres) => Ok(genres),
                        Err(_) => Err("Failed to parse response".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}
