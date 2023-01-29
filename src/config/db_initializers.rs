use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;

pub async fn db_initializers() -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    let database = Database::connect(database_uri).await?;
    Ok(database)
}
