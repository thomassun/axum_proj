use std::sync::Arc;

use axum::extract::FromRef;

// #[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Client,
}

// impl FromRef<AppState> for mongodb::Client {
//     fn from_ref(input: &AppState) -> Self {
//         input.db.clone()
//     }
// }
