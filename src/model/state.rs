// #[derive(Clone)]
pub struct AppState {
    pub db_client: mongodb::Client,
}

// impl FromRef<AppState> for mongodb::Client {
//     fn from_ref(input: &AppState) -> Self {
//         input.db.clone()
//     }
// r
