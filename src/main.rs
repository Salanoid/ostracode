// mod controllers;
// mod models;
pub mod config;

pub use config::routes;
pub use config::environment;
pub use ostracode::server;

#[tokio::main]
async fn main() {
    server().await
}
