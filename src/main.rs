use axum::{response::IntoResponse, routing::get, Json, Router};
// use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user", get(test()));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Clone)]
struct User {
    id: usize,
    name: String,
    age: i32,
}

async fn get_user() -> impl IntoResponse {
    Json(User {
        id: 1,
        name: "Thomas".to_string(),
        age: 9,
    })
}
#[axum_macros::debug_handler]
async fn test() -> String {
    "Test".to_owned()
}
