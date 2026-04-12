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
use embassy_time::Timer;
use embedded_hal_bus::spi::CriticalSectionDevice;
use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::dma_buffers;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::Mode;
use esp_hal::spi::master::Config;
use esp_hal::spi::master::Spi;
use esp_hal::spi::master::SpiDmaBus;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use max31865::{Max31865, Numwires};
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
async fn main(_spawner: Spawner) {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let scl_mosi = peripherals.GPIO11;
    let scl_sck = peripherals.GPIO12;
    let scl_miso = peripherals.GPIO13;

    let max31865_cs = peripherals.GPIO9;
    let max31865_drdy = peripherals.GPIO8;

    let dma_channel = peripherals.DMA_CH0;

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(2048);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let max31865_cs = Output::new(max31865_cs, Level::Low, OutputConfig::default());
    let mut _max31865_drdy = Input::new(max31865_drdy, InputConfig::default().with_pull(Pull::Up));

    let spi_bus = Spi::new(
        peripherals.SPI3,
        Config::default()
            .with_frequency(Rate::from_mhz(5))
            .with_mode(Mode::_1),
    )
    .unwrap()
    .with_sck(scl_sck)
    .with_mosi(scl_mosi)
    .with_miso(scl_miso)
    .with_dma(dma_channel)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let delay = Delay::new();
    let mutex_refcell_bus = MUTEX_SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let max31865_spi_device =
        CriticalSectionDevice::new(mutex_refcell_bus, max31865_cs, delay).unwrap();

    info!("create max31865");
    let mut max31865 = Max31865::new(
        max31865_spi_device,
        delay,
        Numwires::MAX31865_3_WIRE,
        100.0,
        430.0,
    );

    let mut debug_value = max31865.get_debug_value();
    Timer::after_millis(65).await;

    loop {
        info!(
            "temperature: {}, resistance: {}",
            debug_value.temperature, debug_value.resistance
        );

        debug_value = max31865.get_debug_value();

        Timer::after_secs(1).await;
    }
}
