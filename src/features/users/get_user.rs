use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

pub fn get_route() -> Router {
    Router::new().route("/users", get(get_person))
}

pub async fn get_person() -> (StatusCode, Json<GetUser>) {
    let person = GetUser {
        age: 20,
        name: String::from("abbe"),
    };

    (StatusCode::FOUND, Json(person))
}

#[derive(Serialize)]
pub struct GetUser {
    pub name: String,
    pub age: i8,
}