use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal_bus::spi::CriticalSectionDevice;
use embedded_sdmmc::{SdCard, VolumeManager};
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaChannelFor, DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{InputPin, Level, Output, OutputConfig, OutputPin};
use esp_hal::spi::master::{AnySpi, Spi};
use esp_hal::spi::master::{Config, Instance as SpiInstance};
use esp_hal::time::Rate;

use crate::App;
use crate::initializator::{MutexedVolumeManagerType, SPI_BUS, VOLUME_MANAGER};
use crate::tasks::volume_manager_task::DummyTimeSource;

use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default)]
pub struct VolumeManInitConfig<MOSI, SCK, MISO, SDCS, DMA, SPI>
where
    MOSI: OutputPin + 'static,
    SCK: OutputPin + 'static,
    MISO: InputPin + 'static,
    SDCS: OutputPin + 'static,
    SPI: SpiInstance + 'static,
    DMA: DmaChannelFor<AnySpi<'static>>,
{
    mosi_pin: MOSI,
    sck_pin: SCK,
    miso_pin: MISO,
    sd_card_cs: SDCS,
    dma: DMA,
    spi: SPI,
}

pub trait VolumeManInitExtension {
    fn init_volume_manager<MOSI, SCK, MISO, SDCS, DMA, SPI>(
        config: VolumeManInitConfig<MOSI, SCK, MISO, SDCS, DMA, SPI>,
    ) -> &'static MutexedVolumeManagerType
    where
        MOSI: OutputPin + 'static,
        SCK: OutputPin + 'static,
        MISO: InputPin + 'static,
        SDCS: OutputPin + 'static,
        SPI: SpiInstance + 'static,
        DMA: DmaChannelFor<AnySpi<'static>>;
}

impl VolumeManInitExtension for App {
    fn init_volume_manager<MOSI, SCK, MISO, SDCS, DMA, SPI>(
        config: VolumeManInitConfig<MOSI, SCK, MISO, SDCS, DMA, SPI>,
    ) -> &'static MutexedVolumeManagerType
    where
        MOSI: OutputPin + 'static,
        SCK: OutputPin + 'static,
        MISO: InputPin + 'static,
        SDCS: OutputPin + 'static,
        SPI: SpiInstance + 'static,
        DMA: DmaChannelFor<AnySpi<'static>>,
    {
        // let l = FooBuilder
        init_volume_manager(
            config.mosi_pin,
            config.sck_pin,
            config.miso_pin,
            config.sd_card_cs,
            config.dma,
            config.spi,
        )
    }
}

/// REFERENCES:
/// - https://docs.espressif.com/projects/rust/esp-hal/1.1.1/esp32s3/esp_hal/dma/trait.DmaChannelFor.html
fn init_volume_manager<DMA, SPI>(
    mosi_pin: impl OutputPin + 'static,
    sck_pin: impl OutputPin + 'static,
    miso_pin: impl InputPin + 'static,
    sd_card_cs: impl OutputPin + 'static,
    dma: DMA,
    spi: SPI,
) -> &'static MutexedVolumeManagerType
where
    SPI: SpiInstance + 'static,
    DMA: DmaChannelFor<AnySpi<'static>>,
{
    let microsd_cs = Output::new(sd_card_cs, Level::Low, OutputConfig::default());

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(2048);

    let dma_rx_buf =
        DmaRxBuf::new(rx_descriptors, rx_buffer).expect("fail to allocaet spi dma rx buffer");

    let dma_tx_buf =
        DmaTxBuf::new(tx_descriptors, tx_buffer).expect("fail to allocaet spi dma tx buffer");

    let spi_bus = Spi::new(
        spi,
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
    let mutex_refcell_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi_bus)));

    let spi_mmc_device = CriticalSectionDevice::new(mutex_refcell_bus, microsd_cs, delay).unwrap();

    let sdcard = SdCard::new(spi_mmc_device, delay);
    let volman = VolumeManager::new(sdcard, DummyTimeSource);

    VOLUME_MANAGER.init(Mutex::new(RefCell::new(volman)))
}
