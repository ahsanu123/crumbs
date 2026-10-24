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
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_hal_bus::spi::CriticalSectionDevice;
use embedded_sdmmc::SdCard;
use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::dma_buffers;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::Mode;
use esp_hal::spi::master::Config;
use esp_hal::spi::master::Spi;
use esp_hal::spi::master::SpiDmaBus;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use exfat_slim::asynchronous::file::OpenOptions;
use exfat_slim::asynchronous::fs;
use static_cell::StaticCell;
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

pub static SPI_BUS: StaticCell<Mutex<RefCell<SpiDmaBus<'static, Blocking>>>> = StaticCell::new();

type SpiType = CriticalSectionDevice<'static, SpiDmaBus<'static, Blocking>, Output<'static>, Delay>;

static DUMMY_STR: &str = "Resetting to the beginning: Using file.seek(0) sends the cursor back to the start of the file so you can re-read or re-process its contents.";

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

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let scl_mosi = peripherals.GPIO11;
    let scl_sck = peripherals.GPIO12;
    let scl_miso = peripherals.GPIO13;

    let sdcard_cs = peripherals.GPIO18;

    let dma_channel = peripherals.DMA_CH0;

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(2048);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let sdcard_cs = Output::new(sdcard_cs, Level::High, OutputConfig::default());

    let spi_bus = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(5))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(scl_sck)
    .with_mosi(scl_mosi)
    .with_miso(scl_miso)
    .with_dma(dma_channel)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let delay = Delay::new();
    let mutex_refcell_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let spi_dev = CriticalSectionDevice::new(mutex_refcell_bus, sdcard_cs, delay).unwrap();
    let sdcard = SdCard::new(spi_dev, delay);

    defmt::info!("spawning file_system_task");
    spawner.spawn(file_system_task(sdcard)).unwrap();
    Timer::after_millis(100).await;

    loop {
        defmt::info!("attempt to read file");

        let openopt = OpenOptions {
            read: true,
            write: true,
            append: true,
            truncate: false,
            create: false,
            create_new: false,
        };

        let file = fs::open("dummy.txt", openopt)
            .await
            .expect("fail to openfile");

        for word in DUMMY_STR.split(' ') {
            defmt::info!("writing {}", word);
            file.write(word.as_bytes()).await.unwrap();
        }

        file.close().await.unwrap();

        defmt::info!("=======================================\n\n");
        Timer::after(embassy_time::Duration::from_secs(2)).await;
    }
}

#[embassy_executor::task()]
async fn file_system_task(sdcard: SdCard<SpiType, Delay>) {
    agnostic_mmc::file_system_task_from_sdcard(sdcard).await;
    defmt::info!("file_system_task ended");
}
