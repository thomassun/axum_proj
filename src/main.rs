use axum::{routing::get, Router};
use axum_example::route::user::user_route;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .merge(user_route());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 0.0.0.0:3000....");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
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
        *elem = 12;
        assert_eq!(v, vec![12, 2, 3]);
    }
}
