use shared::db;
use shared::logger;
use shared::settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::initialise();
    let settings = Settings::load()?;
    let _pool = db::create_connection_pool(&settings.database_settings).await?;
    Ok(())
}
