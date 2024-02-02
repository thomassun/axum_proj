use axum::{http::StatusCode, routing::post, Json, Router};

use crate::model::{
    user::{CreateUser, User},
    ValidatedJson,
};

pub fn user_route() -> Router {
    Router::new().route("/users", post(create_user))
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
