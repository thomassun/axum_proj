use axum::{extract::Path, routing::get, Router};
use axum_example::route::user::user_route;
use tracing::info;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/:p0/:p1", get(root))
        .merge(user_route());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on 0.0.0.0:3000....");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(Path(pair): Path<(u32, i32)>) -> &'static str {
    println!("{:?}", pair);
    "Hello, World!"
    // pair
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {
        let mut v = [Err("error!"), Ok(1), Ok(2), Ok(3), Err("foo")];
        let w = [Ok('c'), Err("error!"), Ok('2'), Ok('3'), Err("foo")];
        for v_i in &mut v {
            *v_i = Ok(111) as Result<i32, &str>;
            // println!("{v_i:?}");
        }
        // let z = v;
        println!("{w:?}");
        println!("{v:?}");
        assert_eq!(Some(1).and(Some(2)), Some(2));
    }
}
