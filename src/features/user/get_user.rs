use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use crate::domain::entities::user::User;

pub fn get_route() -> Router {
    Router::new().route("/user", get(get_person))
}

pub async fn get_person() -> (StatusCode, Json<User>) {
    let person = User{
        age: 20,
        name: String::from("abbe")
    };

    (StatusCode::FOUND, Json(person))
}