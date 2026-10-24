#![no_std]
#![no_main]

use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::rtc_cntl::Rtc;
use log::LevelFilter;
use plant_sensor::config;
use plant_sensor::reset;
use plant_sensor::reset::SleepReason;

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

/// Log and reset system when flashed software panics
#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    log::error!("panicked: {}", panic_info.message());
    esp_hal::system::software_reset();
}

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
    let led = Output::new(peripherals.GPIO2, Level::Low, output_config);

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
                reset::sleep_deep(rtc, delay, SleepReason::Error { led });
            }
        };
        total_adc += u32::from(recorded_adc);
        delay.delay_millis(config::ADC_POLL_INTERVAL_MS);
    }
    let average_adc = total_adc / config::ADC_POLL_COUNT;
    log::info!("Average adc is {}", average_adc);

    reset::sleep_deep(rtc, delay, SleepReason::Success);
}
