use shared::logger;
use shared::settings::Settings;

fn main() -> anyhow::Result<()> {
    logger::initialise();
    let _settings = Settings::load()?;
    Ok(())
}
