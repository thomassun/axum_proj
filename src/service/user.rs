use std::sync::Arc;

use anyhow::Ok;
use axum::{
    extract::State,
    http::{header, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_macros::debug_handler;
use jsonwebtoken::{encode, EncodingKey};
use mongodb::bson::{doc, Document};
use serde_json::json;

use crate::model::{
    errors::AppError,
    state::AppState,
    user::{Claims, CreateUser, LoginUserSchema, User},
    ValidatedJson,
};

pub fn user_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", post(create_user))
        .route("/api/login", post(login_user_handler))
        .route("/api/me", get(me))
}
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    ValidatedJson(Json(payload)): ValidatedJson<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response aka **Serialize
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[debug_handler]
async fn login_user_handler(
    Json(user_credential): Json<LoginUserSchema>,
    // ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
) -> Result<impl IntoResponse, AppError> {
    // println!("{r:#?}");
    if user_credential.pwd != "test" {
        // Err(AppError::Unauthorized (json!({"status":"-11","message":"invalid credential"}) }"}) }"}) })))
        Err(AppError::Unauthorized(
            json!({"code": "-11","message":"invalid credential"}).into(),
        ))
    } else {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let claims = Claims {
            sub: user_credential.user,
            exp,
            iat: now.timestamp() as usize,
            payload: None,
        };
        let token = encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(b"secret"),
        )
        .unwrap();
        let cookie = Cookie::build(("token", token.clone()))
            .path("/")
            .max_age(time::Duration::hours(1))
            .same_site(SameSite::Lax)
            .http_only(true);
        // .finish();
        let body = json!({"status":"OK","jwt_token": token});
        // let mut new_headers = HeaderMap::new();
        // new_headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
        core::result::Result::Ok((
            StatusCode::OK,
            [
                (
                    HeaderName::from_lowercase("my-header".as_bytes()).unwrap(),
                    "MY Header value".parse::<HeaderValue>().unwrap(),
                ),
                (
                    header::SET_COOKIE,
                    cookie.to_string().parse::<HeaderValue>().unwrap(),
                ),
            ],
            Json(body),
        ))
    }
}

#[debug_handler]
async fn me(user: User, mongodb: State<Arc<AppState>>) -> Result<Json<User>, AppError> {
    // mongodb
    //     .db_client
    //     .list_databases(None, None)
    //     .await
    //     .unwrap()
    //     .iter()
    //     .for_each(|db| println!("{db:?}"));
    let db = mongodb.db_client.database("mindpilot");
    let doc = db.collection::<Document>("doc");
    println!(
        "{:?}",
        doc.find_one(doc! {"meta.name":"SGArrivalCard_081020221004.pdf"}, None)
            .await
            .unwrap()
    );
    Ok(Json(user)).map_err(|_| AppError::None)
}
