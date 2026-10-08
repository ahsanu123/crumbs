use core::convert::Infallible;
use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb565, RgbColor},
};
use fish_pond_rust::fish_body_simulation::functions::rgb565;
use fish_pond_rust::{Pond, SceneConfig};

struct NullDisplay {
    size: Size,
    pixels: usize,
}

impl OriginDimensions for NullDisplay {
    fn size(&self) -> Size {
        self.size
    }
}

impl DrawTarget for NullDisplay {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.pixels += pixels.into_iter().count();
        Ok(())
    }
}

impl NullDisplay {
    fn new(size: Size) -> Self {
        Self { size, pixels: 0 }
    }

    fn take_pixels(&mut self) -> usize {
        core::mem::take(&mut self.pixels)
    }
}

#[test]
fn scene_dimensions_and_caller_timing_drive_step() {
    let mut config = SceneConfig::new(320, 240);
    config.fish_count = 1;
    config.leaf_count = 1;
    config.duckweed_count = 1;
    config.seed = 42;
    let mut pond = Pond::new(config);
    let mut display = NullDisplay::new(Size::new(320, 240));

    pond.step(&mut display, 16);
    pond.step(&mut display, 50);

    assert_eq!(pond.width, 320.0);
    assert_eq!(pond.height, 240.0);
    assert_eq!(pond.fishes.len(), 1);
}

#[test]
fn css_components_are_quantized_to_rgb565_ranges() {
    assert_eq!(rgb565(255, 0, 0), Rgb565::RED);
    assert_eq!(rgb565(0, 255, 0), Rgb565::GREEN);
    assert_eq!(rgb565(0, 0, 255), Rgb565::BLUE);
    assert_eq!(rgb565(255, 255, 255), Rgb565::WHITE);

    let pond_blue = rgb565(0, 105, 148);
    assert_eq!((pond_blue.r(), pond_blue.g(), pond_blue.b()), (0, 26, 18));
}

#[test]
fn every_fish_accessory_draws() {
    let mut config = SceneConfig::new(320, 240);
    config.fish_count = 1;
    config.leaf_count = 0;
    config.duckweed_count = 0;
    config.seed = 42;
    let mut pond = Pond::new(config);
    let mut display = NullDisplay::new(Size::new(320, 240));
    pond.step(&mut display, 16);
    display.take_pixels();

    let fish = &pond.fishes[0];
    fish.draw_fins(&mut display, rgb565(30, 30, 30), rgb565(155, 155, 155), 1);
    assert!(display.take_pixels() > 0);

    fish.draw_tail(&mut display, rgb565(30, 30, 30), rgb565(155, 155, 155), 1);
    assert!(display.take_pixels() > 0);

    fish.draw_back_fin(&mut display, rgb565(30, 30, 30), rgb565(155, 155, 155), 1);
    assert!(display.take_pixels() > 0);

    fish.draw_eyes(&mut display, rgb565(155, 155, 155), rgb565(192, 30, 57));
    assert!(display.take_pixels() > 0);
}

#[test]
fn ripple_growth_is_proportional_to_scene_size() {
    let mut large_config = SceneConfig::new(960, 640);
    large_config.fish_count = 0;
    large_config.leaf_count = 0;
    large_config.duckweed_count = 0;
    large_config.seed = 42;
    let mut small_config = large_config;
    small_config.width = 480;
    small_config.height = 320;

    let mut large = Pond::new(large_config);
    let mut small = Pond::new(small_config);
    large.add_ripple(100.0, 100.0, 100.0);
    small.add_ripple(50.0, 50.0, 100.0);
    let mut large_display = NullDisplay::new(Size::new(960, 640));
    let mut small_display = NullDisplay::new(Size::new(480, 320));

    large.step(&mut large_display, 16);
    small.step(&mut small_display, 16);

    let large_radius = large.ripples[0].ripple_group[0].target_radius;
    let small_radius = small.ripples[0].ripple_group[0].target_radius;
    assert_eq!(small.spatial_scale, large.spatial_scale / 2.0);
    assert_eq!(small_radius, large_radius / 2.0);
}

#[test]
fn slow_or_skipped_rendering_does_not_age_entities_away() {
    let mut config = SceneConfig::new(160, 128);
    config.fish_count = 1;
    config.leaf_count = 0;
    config.duckweed_count = 0;
    config.seed = 42;
    let mut pond = Pond::new(config);
    pond.add_ripple(80.0, 64.0, 100.0);

    pond.update(10_000);

    assert_eq!(pond.fishes.len(), 1);
    assert!(!pond.ripples.is_empty());
    assert!(!pond.ripples[0].is_empty());
    assert!(pond.ripples[0].ripple_group[0].current_radius > 0.0);
}
