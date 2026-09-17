use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal_bus::spi::CriticalSectionDevice;
use embedded_sdmmc::SdCard;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaChannelFor, DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{InputPin, Level, Output, OutputConfig, OutputPin};
use esp_hal::spi::master::{AnySpi, Spi};
use esp_hal::spi::master::{Config, Instance as SpiInstance};
use esp_hal::time::Rate;

use crate::App;
use crate::initializator::MUTEX_SPI_BUS;

pub trait SdCardInitializerExtension {
    fn init_sd_card<DMA, SPI>(
        &mut self,
        mosi_pin: impl OutputPin + 'static,
        sck_pin: impl OutputPin + 'static,
        miso_pin: impl InputPin + 'static,
        sd_card_cs: impl OutputPin + 'static,
        dma: DMA,
        spi: SPI,
    ) where
        SPI: SpiInstance + 'static,
        DMA: DmaChannelFor<AnySpi<'static>>;
}

impl SdCardInitializerExtension for App {
    fn init_sd_card<DMA, SPI>(
        &mut self,
        mosi_pin: impl OutputPin + 'static,
        sck_pin: impl OutputPin + 'static,
        miso_pin: impl InputPin + 'static,
        sd_card_cs: impl OutputPin + 'static,
        dma: DMA,
        spi: SPI,
    ) where
        SPI: SpiInstance + 'static,
        DMA: DmaChannelFor<AnySpi<'static>>,
    {
        init_sd_card(mosi_pin, sck_pin, miso_pin, sd_card_cs, dma, spi);
    }
}

/// REFERENCES:
/// - https://docs.espressif.com/projects/rust/esp-hal/1.1.1/esp32s3/esp_hal/dma/trait.DmaChannelFor.html
pub fn init_sd_card<DMA, SPI>(
    mosi_pin: impl OutputPin + 'static,
    sck_pin: impl OutputPin + 'static,
    miso_pin: impl InputPin + 'static,
    sd_card_cs: impl OutputPin + 'static,
    dma: DMA,
    spi: SPI,
) where
    SPI: SpiInstance + 'static,
    DMA: DmaChannelFor<AnySpi<'static>>,
{
    let microsd_cs = Output::new(sd_card_cs, Level::Low, OutputConfig::default());

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(2048);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_bus = Spi::new(
        spi,
        // peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(5))
            .with_mode(esp_hal::spi::Mode::_1),
    )
    .expect("fail to create spi device")
    .with_sck(sck_pin)
    .with_mosi(mosi_pin)
    .with_miso(miso_pin)
    .with_dma(dma)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let delay = Delay::new();
    let mutex_refcell_bus = MUTEX_SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let spi_mmc_device = CriticalSectionDevice::new(mutex_refcell_bus, microsd_cs, delay).unwrap();
    // TODO: move this to static cell
    let sdcard = SdCard::new(spi_mmc_device, delay);
}
