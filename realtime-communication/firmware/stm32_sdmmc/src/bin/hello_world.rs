#![no_std]
#![no_main]

extern crate alloc;

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::Hertz;
use embassy_stm32::{
    bind_interrupts, dma, peripherals,
    sdmmc::{self, Sdmmc},
};
use embassy_time::{Duration, Timer};
use embedded_alloc::LlffHeap as Heap;
use exfat_slim::asynchronous::fs;
use stm32_sdmmc::sd_block_dev;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SDIO => sdmmc::InterruptHandler<peripherals::SDIO>;
    DMA2_STREAM3 => dma::InterruptHandler<peripherals::DMA2_CH3>;
});

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 4096;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    defmt::info!("init alloc");
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_SIZE);
    }
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 168 / 2 = 168Mhz.
            divq: Some(PllQDiv::DIV7), // 8mhz / 4 * 168 / 7 = 48Mhz.
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }

    let p = embassy_stm32::init(config);

    defmt::info!("setup led");

    let mut led = Output::new(p.PA1, Level::Low, Speed::Medium);

    defmt::info!("before loop");
    loop {
        led.toggle();
        defmt::info!("toggled led");
        Timer::after(Duration::from_secs(1)).await;
    }
}
