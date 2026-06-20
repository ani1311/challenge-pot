use std::sync::Arc;

use challenge_pot_server::{infrastructure::fs_persistence::FsPersistence, presentation::{self, http::AppState}};

const DATA_DIR: &str = "/tmp/challenge-pot";

#[tokio::main]
async fn main() {
    let state = AppState{
        persistence: Arc::new(FsPersistence::new(DATA_DIR.into()))
    };

    let app= presentation::http::router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002")
        .await
        .expect("bind server address");

    axum::serve(listener, app)
        .await
        .expect("run server")
}
