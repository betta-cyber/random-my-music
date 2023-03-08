extern crate redis;

use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Path, Query},
    http::{self, header, request::Parts, StatusCode, Method},
    // middleware::from_extractor,
    response::IntoResponse,
    routing::{get, post, get_service},
    Extension, Json, Router,
};
use axum_sessions::{
    async_session::MemoryStore,
    extractors::{ReadableSession, WritableSession},
    SessionLayer,
};
use headers::{HeaderName, HeaderValue};
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use sqlx::mysql::{MySql, MySqlPool, MySqlPoolOptions};
use std::collections::HashMap;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;


#[derive(Clone)]
struct MyShared {
    db: MySqlPool,
    redis: Client,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct RequireAuth {
    user: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync + std::fmt::Debug,
{
    type Rejection = http::StatusCode;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = req
            .headers
            .get(header::AUTHORIZATION)
            // .and_then(|headers| headers.get(axum::http::header::AUTHORIZATION))
            .and_then(|value| value.to_str().ok());

        if let Some(value) = auth_header {
            if value == "secret" {
                return Ok(Self {
                    user: Some("sssss".to_string()),
                });
            }
        }
        Ok(Self { user: None })
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "mysql://root:9eHp7GqMEGkAq0C2IGZz@containers-us-west-54.railway.app:6021/rym_music"
            .to_string()
    });

    // setup connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let redis = redis::Client::open(
        "redis://default:tCnRdLhEz1qt3IKJuRDX@containers-us-west-145.railway.app:5767",
    )
    .expect("can't connect to redis");

    let cors = CorsLayer::new()
                // .allow_origin("http://0.0.0.0:5001".parse::<HeaderValue>().unwrap())
                .allow_origin(Any)
                // .allow_origin("https://randomyourmusic.fun".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
                .allow_credentials(false);

    let store = MemoryStore::new();
    let secret = b"zgn7ryv4yuzghfzr48903m77qm4pz4xilh10toep1pgxhebkzvp2nfmodwxv7ug2";
    let session_layer = SessionLayer::new(store, secret)
        .with_secure(false);

    let api = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/user_config", post(user_config))
        .route("/user", get(user_info))
        .route("/today", get(get_today_album))
        .route("/album/:album_id", get(get_album_detail))
        .route("/genres", get(genres))
        .layer(cors)
        // .route_layer(from_extractor::<RequireAuth>())
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        ))
        .layer(session_layer)
        .layer(Extension(MyShared {
            db: pool,
            redis,
        }));

    let static_files_service = get_service(
        ServeDir::new("../dist")
        .fallback(ServeFile::new("../dist/index.html"))
        // .append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });
    // build our application with a route
    let app = Router::new()
        .fallback(static_files_service)
        .nest("/api/v1", api);


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 5001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[derive(Deserialize)]
pub struct SubjectArgs {
    pub client_id: String,
}

async fn get_today_album(
    Query(args): Query<SubjectArgs>,
    session: ReadableSession,
    Extension(state): Extension<MyShared>,
) -> impl IntoResponse {

    let client_id = args.client_id.to_string();
    let mut con = state.redis.get_async_connection().await.unwrap();
    let res: String = con.get(&client_id).await.unwrap_or_default();
    if res.is_empty() {
        let fresh_time: usize = session.get("fresh_time").unwrap_or_default();
        let user_genres: String = session.get("user_genres").unwrap_or_default();
        let album_list = if user_genres.is_empty() {
            sqlx::query_as::<MySql, Album>(
                r#"SELECT r1.id, name, cover
                FROM album AS r1 where locate("cdn", r1.cover) ORDER BY rand() ASC LIMIT 40"#,
            )
            .fetch_all(&state.db)
            .await
        } else {
            let genre_list = user_genres.split(",");
            let mut search_query = String::new();
            for genre in genre_list {
                let g = format!(" or r2.genre like '%{}%'", genre);
                search_query.push_str(&g);
            }
            let search_query = &search_query[3..];
            let search_query = format!("({})", search_query);
            let sql = format!(r#"SELECT r1.id, name, cover FROM album AS r1 right join album_genre r2
            on r1.id = r2.album_id where locate("cdn", r1.cover) and {}
            ORDER BY rand() ASC LIMIT 40"#, search_query);
            sqlx::query_as::<MySql, Album>(&sql)
                .fetch_all(&state.db)
                .await
        };
        if let Ok(album_list) = album_list {
            let json = serde_json::to_string(&album_list).unwrap();
            let _: () = con.set_ex(&client_id, &json, fresh_time*60).await.unwrap();
            return json
        } else {
            "error".to_string()
        }
    } else {
        return res
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct Album {
    id: i32,
    name: String,
    // artist: String,
    cover: String,
    // media_url: sqlx::types::Json<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct AlbumDetail {
    id: i32,
    name: String,
    artist: String,
    cover: String,
    media_url: sqlx::types::Json<HashMap<String, serde_json::Value>>,
    descriptors: String,
    language: String,
    rate: String,
    released: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow, Default)]
pub struct AlbumGenre {
    genre: String,
    genre_type: String,
}

async fn get_album_detail(
    Path(album_id): Path<u64>,
    // session: ReadableSession,
    Extension(state): Extension<MyShared>,
) -> impl IntoResponse {
    // println!("{session:?}");
    // let a: String = session.get("username").unwrap_or_default();
    // println!("{:#?}", a);

    let sql = format!(
        r#"SELECT a.id, a.name, a.artist, a.cover, a.media_url, b.descriptors, b.released,
    b.language, b.rate from album a left join album_detail b on a.id = b.album_id where a.id = {album_id}"#
    );
    let album_detail = sqlx::query_as::<MySql, AlbumDetail>(&sql)
        .fetch_one(&state.db)
        .await
        .unwrap();
    let genre_sql = format!(
        r#"SELECT genre, genre_type from album_genre where album_id = {album_id}"#
    );
    let genres = sqlx::query_as::<MySql, AlbumGenre>(&genre_sql)
        .fetch_all(&state.db)
        .await
        .unwrap();
    let mut j = serde_json::to_value(&album_detail).unwrap();
    j["genres"] = serde_json::to_value(genres).unwrap();
    Json(j)
}

async fn generate_password(password: &str) -> String {
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();
    // write input message
    hasher.update(password);

    let result: String = format!("{:X}", hasher.finalize());
    result
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
    password: String,
    password_confirm: String,
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
    #[sqlx(default)]
    session_id: Option<String>,
    #[sqlx(default)]
    genre_data: Option<String>,
    fresh_time: i32,
}

async fn login(
    Extension(state): Extension<MyShared>,
    mut session: WritableSession,
    Json(payload): Json<Login>,
) -> impl IntoResponse {
    let sql = format!(
        r#"SELECT id, username, email, password, session_id, genre_data, fresh_time from rym_user where
                      username = "{}""#,
        payload.username
    );
    match sqlx::query_as::<MySql, User>(&sql)
        .fetch_one(&state.db)
        .await
    {
        Ok(exist_user) => {
            let password = generate_password(&payload.password).await;
            if password == exist_user.password {
                // login
                session.insert("user_id", &exist_user.id).unwrap();
                session.insert("user_genres", &exist_user.genre_data).unwrap();
                session.insert("fresh_time", &exist_user.fresh_time).unwrap();
                let resp = serde_json::json!({
                    "code": 200,
                    "msg": "login success",
                    "data": exist_user
                });
                // let res = res_j.as_str();
                return (StatusCode::OK, Json(resp));
            }
            let resp = serde_json::json!({
                "code": 400,
                "msg": "login failed"
            });
            (StatusCode::OK, Json(resp))
        }
        Err(e) => {
            println!("error {e:#?}");
            let resp = serde_json::json!({
                "code": 400,
                "msg": "login failed"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}

async fn register(
    Extension(state): Extension<MyShared>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here

    if payload.password_confirm != payload.password {
        let resp = serde_json::json!({
            "code": 400,
            "msg": "error"
        });
        let resp = serde_json::to_string(&resp).unwrap();
        return (StatusCode::BAD_REQUEST, resp)
    }
    let sql = format!(
        r#"SELECT id, username, email, password, session_id from rym_user where
                      username = "{}" and email = "{}""#,
        payload.username, payload.email
    );
    match sqlx::query_as::<MySql, User>(&sql)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {
            // println!("found user {exist_user:#?}");
            let resp = serde_json::json!({
                "code": 400,
                "msg": "error"
            });
            let resp = serde_json::to_string(&resp).unwrap();
            return (StatusCode::BAD_REQUEST, resp)
        }
        Err(_) => {
            // println!("no user {e:#?}");
            let password = generate_password(&payload.password).await;
            let insert_sql = format!(
                r#"INSERT INTO rym_user (username, email, password, fresh_time) VALUES ("{}",
            "{}", "{}", 10) "#,
                payload.username, payload.email, password
            );
            match sqlx::query(&insert_sql).execute(&state.db).await {
                Ok(_) => {
                    let resp = serde_json::json!({
                        "code": 200,
                        "msg": "register success",
                        "data": {}
                    });
                    let resp = serde_json::to_string(&resp).unwrap();
                    return (StatusCode::OK, resp)
                }
                Err(_) => {
                    let resp = serde_json::json!({
                        "code": 400,
                        "msg": "error"
                    });
                    let resp = serde_json::to_string(&resp).unwrap();
                    return (StatusCode::BAD_REQUEST, resp)
                }
            }
        }
    }
}


async fn user_info(
    Extension(state): Extension<MyShared>,
    session: ReadableSession,
) -> impl IntoResponse {

    let user_id: i32 = session.get("user_id").unwrap_or_default();
    let sql = format!(
        r#"SELECT id, username, email, password, session_id, genre_data, fresh_time from rym_user where
                      id = "{}""#,
       user_id
    );
    match sqlx::query_as::<MySql, User>(&sql)
        .fetch_one(&state.db)
        .await
    {
        Ok(exist_user) => {
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": exist_user
            });
            return (StatusCode::OK, Json(resp));
        }
        Err(_) => {
            let resp = serde_json::json!({
                "code": 400,
                "msg": "you are not logged in"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Genre {
    id: i32,
    name: String,
    key_name: String,
}

async fn genres(
    Extension(state): Extension<MyShared>,
) -> impl IntoResponse {

    let sql = format!(
        r#"select id, name, key_name from genre where parents = """#
    );
    match sqlx::query_as::<MySql, Genre>(&sql)
        .fetch_all(&state.db)
        .await
    {
        Ok(genres) => {
            // println!("found user {genres:#?}");
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "genres": genres
                }
            });
            return (StatusCode::OK, Json(resp));
        }
        Err(_e) => {
            // println!("no user {e:#?}");
            let resp = serde_json::json!({
                "code": 400,
                "msg": "failed"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}


#[derive(Deserialize)]
struct UserConfig {
    // genres: Vec<String>,
    genres: String,
    fresh_time: String,
}


async fn user_config(
    Extension(state): Extension<MyShared>,
    session: ReadableSession,
    Json(payload): Json<UserConfig>,
) -> impl IntoResponse {

    let user_id: i32 = session.get("user_id").unwrap_or_default();
    let sql = format!(
        r#"update rym_user set genre_data = '{}', fresh_time = '{}' where id = {}"#,
        &payload.genres, payload.fresh_time, user_id,
    );
    // println!("{:#?}", sql);
    match sqlx::query(&sql).execute(&state.db).await {
        Ok(res) => {
            println!("res {:#?}", res);
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": {}
            });
            return (StatusCode::OK, Json(resp));
        }
        Err(e) => {
            println!("no user {e:#?}");
            let resp = serde_json::json!({
                "code": 400,
                "msg": "failed"
            });
            return (StatusCode::BAD_REQUEST, Json(resp));
        }
    }
}
