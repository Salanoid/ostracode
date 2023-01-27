mod config;
use config::environment::app_url;
use config::routes::routes;

pub async fn server() {
    // TODO connect to db     
    axum::Server::bind(&app_url().parse().unwrap())
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
