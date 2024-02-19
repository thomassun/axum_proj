use std::collections::HashMap;

use axum::{
    extract::{Path, Request},
    http::{
        header::{self, CONTENT_TYPE},
        HeaderMap, HeaderName, HeaderValue, StatusCode,
    },
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey};
use serde_json::json;

use crate::model::{
    user::{CreateUser, JWTToken, LoginUserSchema, User},
    ValidatedJson,
};

pub fn user_route() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/api/login", post(login_user_handler))
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

async fn login_user_handler(
    Json(user_credential): Json<LoginUserSchema>,
    // ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
) -> Result<impl IntoResponse, impl IntoResponse> {
    // println!("{r:#?}");
    if user_credential.pwd != "test" {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"status":"-11" ,"message":"invalid credential"})),
        ))
    } else {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let claims = JWTToken {
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
        Ok((
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
