use esp_hal::{clock::CpuClock, peripherals::Peripherals};

pub fn init_peripheral() -> Peripherals {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    esp_hal::init(config)
}
