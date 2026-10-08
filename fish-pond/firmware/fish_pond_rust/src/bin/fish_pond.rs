#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::Instant;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_println as _;

use fish_pond_rust::{Pond, SceneConfig};

extern crate alloc;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 128;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32s3 -o alloc -o defmt -o unstable-hal -o esp-backtrace -o embassy -o wifi -o neovim

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_240MHz);

    let peripherals = esp_hal::init(config);

    let (_key_up, _key_left, _key_down, _key_right, _input_is_charging, mut display, mut lcd_blk) =
        fish_pond_rust::initializers::init_devices::init_devices(peripherals);

    let _ = spawner;

    lcd_blk.set_high();
    let mut config = SceneConfig::new(WIDTH, HEIGHT);
    config.seed = 7u64;

    let mut pond = Pond::new(config);
    let mut previous = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed_ms = now
            .duration_since(previous)
            .as_millis()
            .min(u32::MAX as u64) as u32;
        previous = now;
        pond.step(&mut display, elapsed_ms);
        display.flush().unwrap();

        Timer::after_millis(1).await;
    }
}
