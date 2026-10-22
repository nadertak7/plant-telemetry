#![no_std]
#![no_main]

mod config;

use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::rtc_cntl::Rtc;
use esp_hal::rtc_cntl::sleep::TimerWakeupSource;
use log::LevelFilter;

extern crate alloc;

enum SleepReason {
    Success,
    Error,
}

/// Log and reset system when flashed software panics
#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    log::error!("panicked: {}", panic_info.message());
    esp_hal::system::software_reset();
}

/// Blink n times when an error occurs
fn blink_on_error(led: &mut Output, delay: &Delay, rtc: Rtc) -> ! {
    for _ in 0..config::LED_FLASHES_ERROR * 2 {
        led.toggle();
        delay.delay_millis(1000);
    }
    sleep_deep(rtc, &delay, SleepReason::Error)
}

/// Put the microcontroller to deep sleep, wake up after a delay
fn sleep_deep(mut rtc: Rtc, delay: &Delay, reason: SleepReason) -> ! {
    let duration = match reason {
        SleepReason::Success => config::DEEP_SLEEP_DURATION_SUCCESS,
        SleepReason::Error => config::DEEP_SLEEP_DURATION_ERROR,
    };
    let timer = TimerWakeupSource::new(duration);
    log::info!("sleeping for {} seconds", duration.as_secs());
    delay.delay_millis(100);
    rtc.sleep_deep(&[&timer]);
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger(LevelFilter::Info);
    esp_alloc::heap_allocator!(
        #[esp_hal::ram(reclaimed)]
        size: config::RECLAIMED_RAM_KIB * 1024
    );
    let delay = Delay::new();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let output_config = OutputConfig::default();
    let mut led = Output::new(peripherals.GPIO2, Level::Low, output_config);

    let mut adc_config = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin(peripherals.GPIO32, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    let rtc = Rtc::new(peripherals.LPWR);

    let mut total_adc: u32 = 0;
    for i in 0..config::ADC_POLL_COUNT {
        let recorded_adc = match nb::block!(adc.read_oneshot(&mut adc_pin)) {
            Ok(v) => {
                log::info!("recorded ADC attempt {}: {}", i + 1, v);
                v
            }
            Err(e) => {
                log::error!("there was an error recording ADC: {:#?}", e);
                blink_on_error(&mut led, &delay, rtc);
            }
        };
        total_adc += u32::from(recorded_adc);
        delay.delay_millis(config::ADC_POLL_INTERVAL_MS);
    }
    let average_adc = total_adc / config::ADC_POLL_COUNT;
    log::info!("Average adc is {}", average_adc);

    sleep_deep(rtc, &delay, SleepReason::Success);
}
