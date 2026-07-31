use shared::db;
use shared::logger;
use shared::settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::initialise();
    let settings = Settings::new()?;
    let pool = db::create_connection_pool(&settings.database_settings).await?;
    db::run_migrations(&pool).await?;
    println!("Got to this point.");
    Ok(())
}
