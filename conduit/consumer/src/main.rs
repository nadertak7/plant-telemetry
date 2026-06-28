use shared::logger;
use shared::settings::Settings;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    logger::initialise();
    let _ = Settings::load()?;
    Ok(())
}
