use core::convert::Infallible;

use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::Rgb565,
};
use embedded_hal::digital::OutputPin;
use mipidsi::{
    Display,
    interface::{Interface, InterfacePixelFormat},
    models::Model,
};

pub struct FrameBuffer<DISPLAY, const N: usize> {
    pub display: DISPLAY,
    pixels: &'static mut [Rgb565; N],
    width: usize,
    height: usize,
}

impl<DISPLAY, const N: usize> FrameBuffer<DISPLAY, N> {
    pub fn new(
        display: DISPLAY,
        pixels: &'static mut [Rgb565; N],
        width: usize,
        height: usize,
    ) -> Self {
        assert_eq!(N, width * height);
        Self {
            display,
            pixels,
            width,
            height,
        }
    }
}

impl<DI, M, RST, const N: usize> FrameBuffer<Display<DI, M, RST>, N>
where
    DI: Interface,
    M: Model<ColorFormat = Rgb565>,
    Rgb565: InterfacePixelFormat<DI::Word>,
    RST: OutputPin,
{
    pub fn flush(&mut self) -> Result<(), DI::Error> {
        self.display.set_pixels(
            0,
            0,
            (self.width - 1) as u16,
            (self.height - 1) as u16,
            self.pixels.iter().copied(),
        )
    }
}

impl<DISPLAY, const N: usize> OriginDimensions for FrameBuffer<DISPLAY, N> {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl<DISPLAY, const N: usize> DrawTarget for FrameBuffer<DISPLAY, N> {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if point.x >= 0
                && point.y >= 0
                && (point.x as usize) < self.width
                && (point.y as usize) < self.height
            {
                self.pixels[point.y as usize * self.width + point.x as usize] = color;
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.pixels.fill(color);
        Ok(())
    }
}
