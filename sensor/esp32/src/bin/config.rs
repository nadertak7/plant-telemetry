use core::time::Duration;

pub const RECLAIMED_RAM_KIB: usize = 64;
pub const DEEP_SLEEP_SECS_SUCCESS: Duration = Duration::from_secs(5);
pub const DEEP_SLEEP_SECS_ERROR: Duration = Duration::from_secs(10);
pub const LED_FLASHES_ERROR: u8 = 5;
