use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_example::model::{user::{CreateUser, User}, ValidatedJson};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 0.0.0.0:3000....");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
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
#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {
        let v = [Err("error!"), Ok(1), Ok(2), Ok(3), Err("foo")];
        let res: Result<i32, &str> = v.into_iter().sum();
        assert_eq!(res, Err("error!"));
        let v = [Ok(1), Ok(2), Ok(21)];
        let res: Result<i32, &str> = v.into_iter().product();
        assert_eq!(res, Ok(42));
        let mut v = vec![1, 2, 3];
        let elem = v.iter_mut().next().unwrap();
        *elem = 12 ;
        assert_eq!(v, vec![12, 2, 3]);
    }
}
