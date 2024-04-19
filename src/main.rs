use axum::Router;
mod domain;
mod features;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(features::users::get_user::get_route())
        .merge(features::users::add_user::get_route());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
