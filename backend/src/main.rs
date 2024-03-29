extern crate redis;
mod settings;


use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Path, Query},
    http::{self, header, request::Parts, Method, StatusCode},
    // middleware::from_extractor,
    response::IntoResponse,
    routing::{get, get_service, post},
    Extension,
    Json,
    Router,
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
use sqlx::Row;
use std::collections::HashMap;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;
use settings::Settings;


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
    let settings = Settings::new();
    let settings = match settings {
        Ok(settings) => {
            settings
        }
        Err(e) => {
            panic!("config error {}", e);
        }
    };

    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_connection_str = settings.db_url;

    // setup connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let redis = redis::Client::open(settings.redis_url)
    .expect("can't connect to redis");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(false);

    let store = MemoryStore::new();
    let session_layer = SessionLayer::new(store, settings.secret.as_bytes()).with_secure(false);

    let api = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/user_config", post(user_config))
        .route("/user", get(user_info))
        .route("/today", get(get_today_album))
        .route("/album/:album_id", get(get_album_detail))
        .route("/artist/:artist", get(get_artist_album))
        .route("/genres", get(genres))
        .route("/genre/:genre", get(get_genre_album))
        .route("/user_album_log", get(get_user_album_log))
        .layer(cors)
        // .route_layer(from_extractor::<RequireAuth>())
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        ))
        .layer(session_layer)
        .layer(Extension(MyShared { db: pool, redis }));

    let static_files_service = get_service(
        ServeDir::new("../dist").fallback(ServeFile::new("../dist/index.html")), // .append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {error}"),
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
    pagination: Option<Query<Pagination>>,
    session: ReadableSession,
    Extension(state): Extension<MyShared>,
) -> impl IntoResponse {
    let client_id = args.client_id;
    let Query(pagination) = pagination.unwrap_or_default();
    let page_client_id = format!("{}_{}", client_id, pagination.page);

    let mut con = state.redis.get_async_connection().await.unwrap();
    let res: String = con.get(&page_client_id).await.unwrap_or_default();
    if res.is_empty() {
        let (fresh_time, user_genres) = match session.get::<usize>("fresh_time") {
            // try get data in session
            Some(fresh_time) => {
                let user_genres: String = session.get("user_genres").unwrap_or_default();
                (fresh_time, user_genres)
            }
            None => {
                // try get data in database
                let sql = format!(
                    r#"SELECT id, username, email, password, session_id, genre_data, fresh_time from rym_user where
                                  session_id like "%{client_id}%""#
                );
                match sqlx::query_as::<MySql, User>(&sql)
                    .fetch_one(&state.db)
                    .await
                {
                    Ok(user) => {
                        let fresh_time: usize = user.fresh_time as usize;
                        let user_genres: String = user.genre_data.unwrap_or_default();
                        (fresh_time, user_genres)
                    }
                    Err(_) => (10, String::new()),
                }
            }
        };
        // no genres settings
        let album_list = if user_genres.is_empty() {
            let sql = format!(
                r#"SELECT r1.id, name, cover FROM album AS r1 where locate("cdn", r1.cover) ORDER BY rand() ASC LIMIT {}"#,
                pagination.page_size
            );
            sqlx::query_as::<MySql, Album>(&sql)
                .fetch_all(&state.db)
                .await
        } else {
            // use user genres settings
            let search_key = user_genres.replace(',', "|");
            let search_query = format!(
                r#"r2.genre in (select name from genres where path REGEXP '^({search_key})')"#
            );
            let sql = format!(
                r#"SELECT r1.id, name, cover FROM album AS r1 left join album_genre r2
            on r1.id = r2.album_id where locate("cdn", r1.cover) and {}
            ORDER BY rand() ASC LIMIT {}"#,
                search_query, pagination.page_size
            );
            sqlx::query_as::<MySql, Album>(&sql)
                .fetch_all(&state.db)
                .await
        };
        if let Ok(album_list) = album_list {
            // let mut res: Vec<Album> = vec![];
            // if pagination.page > 1 {
            // for i in 1..pagination.page - 1 {
            // let page_client_id = format!("{}_{}", client_id, i);
            // let r: String = con.get(&page_client_id).await.unwrap_or_default();
            // let v: Vec<Album> = serde_json::from_str(&r).unwrap();
            // res.extend(v);
            // }
            // }
            // res.extend(album_list);
            let json = serde_json::to_string(&album_list).unwrap();
            let _: () = con
                .set_ex(&page_client_id, &json, fresh_time * 60)
                .await
                .unwrap();
            json
        } else {
            "error".to_string()
        }
    } else {
        res
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct Album {
    id: i32,
    name: String,
    cover: String,
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
    session: ReadableSession,
    Extension(state): Extension<MyShared>,
) -> impl IntoResponse {
    let sql = format!(
        r#"SELECT a.id, a.name, a.artist, a.cover, a.media_url, IFNULL(b.descriptors, '') as descriptors,
        IFNULL(b.released, '') as released, IFNULL(b.language, '') as language, IFNULL(b.rate, '') as rate
        from album a left join album_detail b on a.id = b.album_id where a.id = {album_id}"#
    );
    let album_detail = sqlx::query_as::<MySql, AlbumDetail>(&sql)
        .fetch_one(&state.db)
        .await;

    match album_detail {
        Ok(detail) => {
            let genre_sql =
                format!(r#"SELECT genre, genre_type from album_genre where album_id = {album_id}"#);
            let genres = sqlx::query_as::<MySql, AlbumGenre>(&genre_sql)
                .fetch_all(&state.db)
                .await
                .unwrap();
            let mut j = serde_json::to_value(&detail).unwrap();
            j["genres"] = serde_json::to_value(genres.clone()).unwrap();

            // insert album log
            let user_id: i32 = session.get("user_id").unwrap_or_default();
            if user_id != 0 {
                let album_genre: String = genres
                    .iter()
                    .map(|g| &*g.genre)
                    .collect::<Vec<&str>>()
                    .join("|");
                let sql = format!(
                    r#"select id, click_count, listen_count from user_album_log where user_id = '{}' and album_id = '{}'"#,
                    user_id, detail.id,
                );
                match sqlx::query(&sql).fetch_one(&state.db).await {
                    Ok(res) => {
                        let update_sql = format!(
                            r#"UPDATE user_album_log set click_count = {}, listen_count
                            = {} WHERE id = {}"#,
                            res.get::<i32, usize>(1) + 1,
                            res.get::<i32, usize>(2) + 1,
                            res.get::<i32, usize>(0)
                        );
                        sqlx::query(&update_sql).execute(&state.db).await.unwrap();
                    }
                    Err(e) => {
                        println!("{e:#?}");
                        let insert_sql = format!(
                            r#"INSERT INTO user_album_log (user_id, album_id, album_genre, click_count,
                            listen_count) VALUES ("{}", "{}", "{}", 1, 1) "#,
                            user_id, detail.id, album_genre
                        );
                        sqlx::query(&insert_sql).execute(&state.db).await.unwrap();
                    }
                }
            }
            (StatusCode::OK, Json(j))
        }
        Err(_) => {
            let resp = serde_json::json!({
                "code": 400,
                "msg": "api error"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
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
    client_id: String,
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
                session.insert("user_id", exist_user.id).unwrap();
                session
                    .insert("user_genres", &exist_user.genre_data)
                    .unwrap();
                session
                    .insert("fresh_time", exist_user.fresh_time)
                    .unwrap();

                // update client session id
                let session_id = match &exist_user.session_id {
                    Some(session_id) => {
                        if session_id.contains(&payload.client_id) {
                            session_id.to_string()
                        } else {
                            format!("{},{}", session_id, payload.client_id)
                        }
                    }
                    None => payload.client_id,
                };
                let update_sql = format!(
                    r#"update rym_user set session_id = "{}" where id = {}"#,
                    session_id, &exist_user.id
                );
                sqlx::query(&update_sql).execute(&state.db).await.unwrap();

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

async fn logout(mut session: WritableSession) -> impl IntoResponse {
    session.destroy();
    let resp = serde_json::json!({
        "code": 200,
        "msg": "logout success",
        "data": {}
    });
    (StatusCode::OK, Json(resp))
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
        return (StatusCode::BAD_REQUEST, resp);
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
            (StatusCode::BAD_REQUEST, resp)
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
                    (StatusCode::OK, resp)
                }
                Err(_) => {
                    let resp = serde_json::json!({
                        "code": 400,
                        "msg": "error"
                    });
                    let resp = serde_json::to_string(&resp).unwrap();
                    (StatusCode::BAD_REQUEST, resp)
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
                      id = "{user_id}""#
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
            (StatusCode::OK, Json(resp))
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

async fn genres(Extension(state): Extension<MyShared>) -> impl IntoResponse {
    let sql = r#"select id, name, key_name from genres where parents = """#.to_string();
    match sqlx::query_as::<MySql, Genre>(&sql)
        .fetch_all(&state.db)
        .await
    {
        Ok(genres) => {
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "genres": genres
                }
            });
            (StatusCode::OK, Json(resp))
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
            println!("res {res:#?}");
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": {}
            });
            (StatusCode::OK, Json(resp))
        }
        Err(e) => {
            println!("no user {e:#?}");
            let resp = serde_json::json!({
                "code": 400,
                "msg": "failed"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct TotalResponse {
    total: i32,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 40,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserAlbumLog {
    album_id: String,
    album_name: String,
    cover: String,
    click_count: i32,
    listen_count: i32,
}

async fn get_user_album_log(
    pagination: Option<Query<Pagination>>,
    Extension(state): Extension<MyShared>,
    session: ReadableSession,
) -> impl IntoResponse {
    let user_id: i32 = session.get("user_id").unwrap_or_default();
    let Query(pagination) = pagination.unwrap_or_default();

    let total_sql = format!(
        "SELECT count(*) AS total FROM user_album_log WHERE user_id = '{user_id}'"
    );
    let total_count = sqlx::query_as::<_, TotalResponse>(&total_sql)
        .fetch_one(&state.db)
        .await
        .unwrap();

    let sql = format!(
        r#"select album_id, r2.name as album_name, r2.cover, click_count, listen_count from
        user_album_log as r1 left join album as r2 on r1.album_id = r2.id
        where r1.user_id = '{}' ORDER BY r1.create_time desc limit {}, {}"#,
        user_id,
        pagination.page_size * (pagination.page - 1),
        pagination.page_size
    );
    match sqlx::query_as::<MySql, UserAlbumLog>(&sql)
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => {
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "res": res,
                    "page": pagination.page,
                    "page_size": pagination.page_size,
                    "total": total_count.total,
                }
            });
            (StatusCode::OK, Json(resp))
        }
        Err(_) => {
            let resp = serde_json::json!({
                "code": 400,
                "msg": "success"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct AlbumChart {
    id: i32,
    name: String,
    artist: String,
    cover: String,
    rate: String,
}

async fn get_genre_album(
    pagination: Option<Query<Pagination>>,
    Path(genre): Path<String>,
    Extension(state): Extension<MyShared>,
    // session: ReadableSession,
) -> impl IntoResponse {
    // let user_id: i32 = session.get("user_id").unwrap_or_default();
    let Query(pagination) = pagination.unwrap_or_default();

    let total_sql = format!("SELECT count(*) AS total FROM album left join album_genre on album.id = album_genre.album_id where genre = '{genre}'");
    let total_count = sqlx::query_as::<_, TotalResponse>(&total_sql)
        .fetch_one(&state.db)
        .await
        .unwrap();

    let sql = format!(
        r#"select r1.id, r1.name, r1.artist, r1.cover, r3.rate from album as r1 left join album_genre as r2 on r1.id = r2.album_id
        left join album_detail as r3 on r1.id = r3.album_id where r2.genre = "{}"
        order by r3.rate desc limit {},{}"#,
        genre,
        pagination.page_size * (pagination.page - 1),
        pagination.page_size
    );
    match sqlx::query_as::<MySql, AlbumChart>(&sql)
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => {
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "res": res,
                    "page": pagination.page,
                    "page_size": pagination.page_size,
                    "total": total_count.total,
                }
            });
            (StatusCode::OK, Json(resp))
        }
        Err(_) => {
            let resp = serde_json::json!({
                "code": 400,
                "msg": "failed"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}

async fn get_artist_album(
    pagination: Option<Query<Pagination>>,
    Path(artist): Path<String>,
    Extension(state): Extension<MyShared>,
    // session: ReadableSession,
) -> impl IntoResponse {
    // let user_id: i32 = session.get("user_id").unwrap_or_default();
    let Query(pagination) = pagination.unwrap_or_default();

    let total_sql = format!(
        "SELECT count(*) AS total FROM album where artist = '{artist}'"
    );
    let total_count = sqlx::query_as::<_, TotalResponse>(&total_sql)
        .fetch_one(&state.db)
        .await
        .unwrap();

    let sql = format!(
        r#"select r1.id, r1.name, r1.artist, r1.cover, IFNULL(r3.rate, '0.00') as rate from album as r1
        left join album_detail as r3 on r1.id = r3.album_id where r1.artist = "{}"
        order by r3.rate desc limit {},{}"#,
        artist,
        pagination.page_size * (pagination.page - 1),
        pagination.page_size
    );
    match sqlx::query_as::<MySql, AlbumChart>(&sql)
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => {
            let resp = serde_json::json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "res": res,
                    "page": pagination.page,
                    "page_size": pagination.page_size,
                    "total": total_count.total,
                }
            });
            (StatusCode::OK, Json(resp))
        }
        Err(_) => {
            let resp = serde_json::json!({
                "code": 400,
                "msg": "failed"
            });
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}
