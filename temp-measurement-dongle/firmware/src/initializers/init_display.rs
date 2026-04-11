use alloc::boxed::Box;
use defmt::info;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::peripherals::Peripherals;
use esp_hal::spi::Mode;
use esp_hal::spi::master::Spi;
use esp_hal::spi::master::{Config, SpiDmaBus};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, dma_buffers};
use mipidsi::Display;
use mipidsi::interface::SpiInterface;
use mipidsi::options::{ColorOrder, Orientation, Rotation};
use slint::ComponentHandle as _;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use static_cell::StaticCell;
use tmd_ui::TmdApp;

use crate::display_models::st7789_modified::ST7789;
use crate::platforms::esp32s3_async::backend::Esp32AsyncBackend;
use crate::platforms::esp32s3_async::draw_buffer::DrawBuffer;
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH, INTERFACE_BUFFER};

pub type DrawBufferType = DrawBuffer<
    Display<
        SpiInterface<
            'static,
            ExclusiveDevice<SpiDmaBus<'static, Blocking>, Output<'static>, Delay>,
            Output<'static>,
        >,
        ST7789,
        Output<'static>,
    >,
>;

pub fn init_display(
    peripherals: Peripherals,
) -> (
    Input<'static>,
    Input<'static>,
    Input<'static>,
    Input<'static>,
    DrawBufferType,
    Output<'static>,
) {
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let input1 = peripherals.GPIO17;
    let input2 = peripherals.GPIO18;
    let input3 = peripherals.GPIO19;
    let input4 = peripherals.GPIO20;

    let lcd_dc = peripherals.GPIO15;
    let lcd_cs = peripherals.GPIO10;
    let lcd_rst = peripherals.GPIO14;
    let lcd_blk = peripherals.GPIO16;

    let scl_mosi = peripherals.GPIO11;
    let scl_sck = peripherals.GPIO12;
    let scl_miso = peripherals.GPIO13;

    let dma_channel = peripherals.DMA_CH0;

    let input1 = Input::new(input1, InputConfig::default().with_pull(Pull::Up));
    let input2 = Input::new(input2, InputConfig::default().with_pull(Pull::Up));
    let input3 = Input::new(input3, InputConfig::default().with_pull(Pull::Up));
    let input4 = Input::new(input4, InputConfig::default().with_pull(Pull::Up));

    let lcd_cs = Output::new(lcd_cs, Level::Low, OutputConfig::default());
    let lcd_dc = Output::new(lcd_dc, Level::Low, OutputConfig::default());
    let lcd_rst = Output::new(lcd_rst, Level::Low, OutputConfig::default());
    let lcd_blk = Output::new(lcd_blk, Level::Low, OutputConfig::default());

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(2048);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_bus = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(60))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(scl_sck)
    .with_mosi(scl_mosi)
    .with_miso(scl_miso)
    .with_dma(dma_channel)
    .with_buffers(dma_rx_buf, dma_tx_buf);

    let interface_buffer = INTERFACE_BUFFER.init([0u8; 312]);

    let mut delay = Delay::new();
    let spi_device = ExclusiveDevice::new(spi_bus, lcd_cs, delay).unwrap();

    let di = mipidsi::interface::SpiInterface::new(spi_device, lcd_dc, interface_buffer);

    let st7789 = mipidsi::Builder::new(crate::display_models::st7789_modified::ST7789, di)
        .display_size(DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16)
        .orientation(Orientation::new().rotate(Rotation::Deg270))
        .color_order(ColorOrder::Rgb)
        .display_offset(34, 340)
        .invert_colors(mipidsi::options::ColorInversion::Inverted)
        .reset_pin(lcd_rst)
        .init(&mut delay)
        .expect("fail to build display st7789");

    let draw_buffer = DrawBuffer::new(st7789);

    (input1, input2, input3, input4, draw_buffer, lcd_blk)
}
