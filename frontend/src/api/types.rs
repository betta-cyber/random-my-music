// use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub genre_data: Option<String>,
    pub fresh_time: i32,
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
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Album {
    pub id: i32,
    // pub name: String,
    pub cover: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AlbumGenre {
    pub genre: String,
    pub genre_type: String,
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
    pub language: String,
    pub rate: String,
    pub genres: Vec<AlbumGenre>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub key_name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AlbumLog {
    pub album_id: String,
    pub album_name: String,
    pub cover: String,
    pub click_count: u32,
    pub listen_count: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AlbumLogData {
    pub res: Vec<AlbumLog>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AlbumChart {
    pub id: u32,
    pub name: String,
    pub artist: String,
    pub cover: String,
    pub rate: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChartData {
    pub res: Vec<AlbumChart>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
}
