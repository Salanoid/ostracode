use axum::http::Method;
#[allow(unused_imports)]
use axum::{
    routing::{get, patch, post, put, delete},
    Router
};
use tower_http::cors::{Any, CorsLayer};

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PATCH, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);
    
    Router::new()
        .route("/", get(root_controller))
        .layer(cors)
}

pub async fn root_controller() -> String {
    "Hello".to_owned()
}
