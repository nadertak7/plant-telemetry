use esp_hal::delay::Delay;
use esp_hal::gpio::Output;
use esp_hal::rtc_cntl::Rtc;
use esp_hal::rtc_cntl::sleep::TimerWakeupSource;

use crate::config;

pub enum SleepReason {
    Success,
    Error,
}

/// Blink n times when an error occurs
pub fn blink_on_error(led: &mut Output, delay: &Delay, rtc: Rtc) -> ! {
    for _ in 0..config::LED_FLASHES_ERROR * 2 {
        led.toggle();
        delay.delay_millis(config::LED_FLASHES_ERROR_INTERVAL_MS);
    }
    sleep_deep(rtc, &delay, SleepReason::Error)
}

/// Put the microcontroller to deep sleep, wake up after a delay
pub fn sleep_deep(mut rtc: Rtc, delay: &Delay, reason: SleepReason) -> ! {
    let duration = match reason {
        SleepReason::Success => config::DEEP_SLEEP_DURATION_SUCCESS,
        SleepReason::Error => config::DEEP_SLEEP_DURATION_ERROR,
    };
    let timer = TimerWakeupSource::new(duration);
    log::info!("sleeping for {} seconds", duration.as_secs());
    delay.delay_millis(100);
    rtc.sleep_deep(&[&timer]);
}
