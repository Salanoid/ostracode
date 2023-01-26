mod controllers;
mod models;
mod config;

pub use config::routes;
pub use ostracode::server;

#[tokio::main]
async fn main() {
    // TODO maybe pass the routes from here
    server().await
}
