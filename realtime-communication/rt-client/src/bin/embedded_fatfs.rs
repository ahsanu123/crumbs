#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cell::RefCell;
use critical_section::Mutex;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::spi::master::SpiDmaBus;
use esp_hal::timer::timg::TimerGroup;
use rt_client::App;
use rt_client::initializator::init_volume_manager::VolumeManInitConfig;
use rt_client::initializator::init_volume_manager::VolumeManInitExtension;
use rt_client::tasks::volume_manager_task::VolumeManagerTask;
use rt_client::tasks::volume_manager_task::run_volume_manager_task;
use static_cell::StaticCell;
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

pub static MUTEX_SPI_BUS: StaticCell<Mutex<RefCell<SpiDmaBus<'static, Blocking>>>> =
    StaticCell::new();

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
    // generator parameters: --chip esp32 -o alloc -o defmt -o unstable-hal -o esp-backtrace -o embassy -o wifi -o neovim

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    let sdcard_config = VolumeManInitConfig::builder()
        .mosi_pin(peripherals.GPIO11)
        .sck_pin(peripherals.GPIO12)
        .miso_pin(peripherals.GPIO13)
        .sd_card_cs(peripherals.GPIO9)
        .dma(peripherals.DMA_SPI2)
        .spi(peripherals.SPI2)
        .build();

    let volume_manager = App::init_volume_manager(sdcard_config);

    let sdcard_task = VolumeManagerTask::builder()
        .volman_instance(volume_manager)
        .build();

    spawner.spawn(run_volume_manager_task(sdcard_task).expect("fail to run sdcard task"));

    loop {
        info!("tick...");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
