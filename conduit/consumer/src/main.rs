use shared::db;
use shared::logger;
use shared::settings::Settings;

mod mqtt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    logger::initialise();
    let settings = Settings::new()?;
    let pool = db::create_connection_pool(&settings.database_settings).await?;
    db::run_migrations(&pool).await?;
    let (mqtt_client, _event_loop) = mqtt::get_client(&settings.mqtt_settings);
    mqtt::subscribe(&mqtt_client, &settings.mqtt_settings).await?;
    println!("Got to this point.");
    Ok(())
}
