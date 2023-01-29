mod config;
use config::environment::app_url;
use config::routes::routes;
use config::db_initializers::db_initializers;

#[allow(unused_must_use)]
pub async fn server() {
    db_initializers().await; 
    axum::Server::bind(&app_url().parse().unwrap())
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
