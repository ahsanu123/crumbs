#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_sdmmc::sdcard::spi::AcquireOpts;
use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::spi::master::SpiDmaBus;
use esp_hal::timer::timg::TimerGroup;
use rt_client::App;
use rt_client::initializator::init_volume_manager::VolumeManInitConfig;
use rt_client::initializator::init_volume_manager::VolumeManInitExtension;
use rt_client::tasks::volume_manager_task::VolumeManagerTask;
use rt_client::tasks::volume_manager_task::run_volume_manager_task;
use static_cell::StaticCell;

use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::delay::DelayNs as _;
use embedded_hal_bus::spi::CriticalSectionDevice;
use embedded_sdmmc::{SdCard, VolumeManager};
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaChannelFor, DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{InputPin, Level, Output, OutputConfig, OutputPin};
use esp_hal::spi::master::{AnySpi, Spi};
use esp_hal::spi::master::{Config, Instance as SpiInstance};
use esp_hal::time::Rate;

use {esp_backtrace as _, esp_println as _};

static SPI_BUS: StaticCell<Mutex<RefCell<SpiDmaBus<'static, Blocking>>>> = StaticCell::new();

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    // .mosi_pin(peripherals.GPIO11)
    // .sck_pin(peripherals.GPIO12)
    // .miso_pin(peripherals.GPIO13)
    // .sd_card_cs(peripherals.GPIO10)
    // .dma(peripherals.DMA_CH0)
    // .spi(peripherals.SPI2)

    let microsd_cs = Output::new(peripherals.GPIO10, Level::High, OutputConfig::default());

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(2048);

    let dma_rx_buf =
        DmaRxBuf::new(rx_descriptors, rx_buffer).expect("fail to allocaet spi dma rx buffer");

    let dma_tx_buf =
        DmaTxBuf::new(tx_descriptors, tx_buffer).expect("fail to allocaet spi dma tx buffer");

    let spi_bus = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(400))
            .with_mode(esp_hal::spi::Mode::_3),
    )
    .expect("fail to create spi device")
    .with_sck(peripherals.GPIO12)
    .with_mosi(peripherals.GPIO11)
    .with_miso(peripherals.GPIO13)
    .with_dma(peripherals.DMA_SPI2)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let delay = Delay::new();
    let mutex_refcell_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let spi_mmc_device = CriticalSectionDevice::new(mutex_refcell_bus, microsd_cs, delay).unwrap();

    // let sdcard = SdCard::new(spi_mmc_device, delay);
    let sdcard = SdCard::new_with_options(
        spi_mmc_device,
        delay,
        AcquireOpts {
            use_crc: false,
            acquire_retries: 50,
        },
    );

    if let Ok(numbyte) = sdcard.num_bytes() {
        defmt::info!("sdcard numbyte{}", numbyte);
    } else {
        defmt::error!("fail to read sdcard numbytes");
    }

    Timer::after(Duration::from_secs(1)).await;
    loop {
        defmt::info!(".");
        Timer::after(Duration::from_secs(10)).await;
    }
}
