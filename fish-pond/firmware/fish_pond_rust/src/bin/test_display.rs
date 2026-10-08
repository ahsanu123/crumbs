#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    primitives::{PrimitiveStyle, Rectangle},
};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_println as _;
use fish_pond_rust::fish_body_simulation::functions::rgb565;

extern crate alloc;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 128;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_240MHz);
    let peripherals = esp_hal::init(config);

    let (_up, _left, _down, _right, _charging, mut display, mut lcd_blk) =
        fish_pond_rust::initializers::init_devices::init_devices(peripherals);
    let _ = spawner;
    lcd_blk.set_high();

    let colors = [
        rgb565(255, 0, 0),
        rgb565(0, 255, 0),
        rgb565(0, 0, 255),
        rgb565(255, 255, 255),
        rgb565(0, 255, 255),
        rgb565(255, 0, 255),
        rgb565(255, 255, 0),
        rgb565(0, 0, 0),
    ];
    let bar_width = WIDTH / colors.len() as u32;

    for (index, color) in colors.into_iter().enumerate() {
        Rectangle::new(
            Point::new(index as i32 * bar_width as i32, 0),
            Size::new(bar_width, HEIGHT),
        )
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(&mut display)
        .unwrap();
    }
    display.flush().unwrap();

    loop {
        Timer::after_secs(1).await;
    }
}
