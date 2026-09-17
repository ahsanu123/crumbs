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
use embedded_hal_bus::spi::CriticalSectionDevice;
use embedded_sdmmc::SdCard;
use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::master::Spi;
use esp_hal::spi::master::{Config, SpiDmaBus};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
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

    let scl_mosi = peripherals.GPIO11;
    let scl_sck = peripherals.GPIO12;
    let scl_miso = peripherals.GPIO13;

    let microsd_cs_pin = peripherals.GPIO9;

    let dma_channel = peripherals.DMA_SPI3;

    let microsd_cs = Output::new(microsd_cs_pin, Level::Low, OutputConfig::default());

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(2048);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_bus = Spi::new(
        peripherals.SPI3,
        Config::default()
            .with_frequency(Rate::from_mhz(5))
            .with_mode(esp_hal::spi::Mode::_1),
    )
    .expect("fail to create spi device")
    .with_sck(scl_sck)
    .with_mosi(scl_mosi)
    .with_miso(scl_miso)
    .with_dma(dma_channel)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let delay = Delay::new();
    let mutex_refcell_bus = MUTEX_SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let spi_mmc_device = CriticalSectionDevice::new(mutex_refcell_bus, microsd_cs, delay).unwrap();
    let sdcard = SdCard::new(spi_mmc_device, delay);

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
