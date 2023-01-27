use dotenvy::dotenv;
use dotenvy_macro::dotenv;

pub fn app_url() -> String {
    dotenv().ok();
    let app_url = dotenv!("APP_URL");
    let app_port = dotenv!("APP_PORT");
    format!("{}:{}", app_url, app_port)
}
