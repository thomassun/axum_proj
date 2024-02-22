use std::sync::Arc;

use axum::{
    extract::{Path, Request},
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use axum_example::{model::state::AppState, service::user::user_route};
use mongodb::{options::ClientOptions, Client};
use tokio::signal;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route

    let mongodb_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    let mongodb = mongodb::Client::with_options(mongodb_options).unwrap();
    let app_stat = Arc::new(AppState {
        db_client: mongodb.clone(),
    });
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/:p0/:p1", get(root))
        .merge(user_route())
        .with_state(app_stat)
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on 0.0.0.0:3000....");
    let main_svc = async {
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(mongodb))
            .await
            .unwrap()
    };
    tokio::join!(main_svc);
    println!("Shutdown the SERVICE at 0.0.0.0:3000....");
}

// basic handler that responds with a static string
async fn root(Path(pair): Path<(u32, i32)>, r: Request) -> &'static str {
    println!("{:?}", pair);
    println!("{:?}", r);
    "Hello, World!"
    // pair
}
async fn shutdown_signal(mongodb: Client) {
    let ctl_c = signal::ctrl_c();
    #[cfg(unix)]
    let terminal = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Unable to install TERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminal = std::future::pending::<()>();
    tokio::select! {
        _ = ctl_c => {println!("CTRL_C")},
            _ = terminal => {println!("SIGTERM")},
    };
    println!("Drop MongoDB connection....");
    mongodb.shutdown().await
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
