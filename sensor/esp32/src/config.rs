use core::time::Duration;

pub const RECLAIMED_RAM_KIB: usize = 64;

pub const DEEP_SLEEP_DURATION_SUCCESS: Duration = Duration::from_secs(5);
pub const DEEP_SLEEP_DURATION_ERROR: Duration = Duration::from_secs(10);
pub const LED_FLASHES_ERROR: u8 = 5;
pub const LED_FLASHES_ERROR_INTERVAL_MS: u32 = 1000;

pub const ADC_POLL_COUNT: u32 = 5;
pub const ADC_POLL_INTERVAL_MS: u32 = 5000;
