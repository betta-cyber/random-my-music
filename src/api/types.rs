// use chrono::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    // pub createdAt: DateTime<Utc>,
    // pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Album {
    pub id: i32,
    pub name: String,
    pub artist: String,
    pub cover: String,
    pub media_url: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct AlbumDetail {
    pub id: i32,
    pub name: String,
    pub artist: String,
    pub cover: String,
    pub media_url: HashMap<String, serde_json::Value>,
    pub descriptors: String,
    pub released: String,
}
