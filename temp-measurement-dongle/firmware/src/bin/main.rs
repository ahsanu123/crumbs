#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use tmd::initializers::{init_devices, init_peripheral};
use tmd::tasks::{input_task, max31865_task, ui_task};

use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let peripherals = init_peripheral();
    let (
        key_up,
        key_left,
        key_down,
        key_right,
        input_is_charging,
        draw_buffer,
        lcd_blk,
        max31865_device,
    ) = init_devices(peripherals);

    spawner
        .spawn(input_task(
            key_up,
            key_right,
            key_down,
            key_left,
            input_is_charging,
        ))
        .expect("fail to start input_task");

    spawner
        .spawn(max31865_task(max31865_device))
        .expect("fail to start input_task");

    ui_task(draw_buffer, lcd_blk).await;
}
