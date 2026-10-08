#![no_std]
#![allow(clippy::module_inception)]

#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub mod display_models;
pub mod fish_body_simulation;
#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub mod initializers;
#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub mod platforms;
pub use fish_body_simulation::sketch::sketch::{ChaseDirection, Pond, SceneConfig};

#[cfg(any(feature = "esp32", feature = "esp32s3"))]
use static_cell::StaticCell;

#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub static LCD_DEV_INTERFACE_BUFFER: StaticCell<[u8; 312]> = StaticCell::new();

#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub static DISPLAY_HEIGHT: usize = 128;
#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub static DISPLAY_WIDTH: usize = 160;
#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub static DISPLAY_PIXELS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

#[cfg(any(feature = "esp32", feature = "esp32s3"))]
pub static FRAME_BUFFER: StaticCell<[embedded_graphics::pixelcolor::Rgb565; DISPLAY_PIXELS]> =
    StaticCell::new();
