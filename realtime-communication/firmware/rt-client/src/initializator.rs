use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal_bus::spi::CriticalSectionDevice;
use embedded_sdmmc::{SdCard, VolumeManager};
use esp_hal::Blocking;
use esp_hal::delay::Delay;
use esp_hal::gpio::Output;
use esp_hal::spi::master::SpiDmaBus;
use static_cell::StaticCell;

use crate::tasks::volume_manager_task::DummyTimeSource;

pub mod init_volume_manager;

pub static SPI_BUS: StaticCell<Mutex<RefCell<SpiDmaBus<'static, Blocking>>>> = StaticCell::new();

pub type SdCardType = SdCard<
    CriticalSectionDevice<'static, SpiDmaBus<'static, Blocking>, Output<'static>, Delay>,
    Delay,
>;
pub type MutexedSdCardType = Mutex<RefCell<SdCardType>>;
pub static SD_CARD: StaticCell<Mutex<RefCell<SdCardType>>> = StaticCell::new();

pub type VolumeManagerType = VolumeManager<SdCardType, DummyTimeSource>;
pub type MutexedVolumeManagerType = Mutex<RefCell<VolumeManagerType>>;
pub static VOLUME_MANAGER: StaticCell<MutexedVolumeManagerType> = StaticCell::new();
