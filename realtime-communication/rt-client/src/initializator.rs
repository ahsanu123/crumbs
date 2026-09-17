use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::Blocking;
use esp_hal::spi::master::SpiDmaBus;
use static_cell::StaticCell;

pub mod init_sd_card;

pub static MUTEX_SPI_BUS: StaticCell<Mutex<RefCell<SpiDmaBus<'static, Blocking>>>> =
    StaticCell::new();
