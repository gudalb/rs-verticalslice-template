use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};

pub fn get_route() -> Router {
    Router::new().route("/users", post(add_person))
}

pub async fn add_person(Json(create_person): Json<CreateUser>) -> (StatusCode, Json<CreateUser>) {
    let person = CreateUser{
        age: create_person.age,
        name: create_person.name
    };

    (StatusCode::CREATED, Json(person))
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name : String,
    pub age : i8,
}