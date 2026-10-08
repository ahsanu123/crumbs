use embedded_graphics::pixelcolor::Rgb565;
use heapless::Vec;

use crate::fish_body_simulation::{
    fish::fish::{Fish, FishScene},
    functions::{Random, rgb565, sqrt},
    leaf::{duckweed::DuckWeed, leaf::Leaf},
};

pub const BACKGROUND_COLOR: Rgb565 = rgb565(0, 105, 148);
pub const FISH_COLOR: Rgb565 = rgb565(192, 30, 57);
pub const EYE_COLOR: Rgb565 = rgb565(0, 0, 0);
pub const FISH_FIN_COLOR: Rgb565 = rgb565(30, 30, 30);
pub const FISH_OUTLINE_COLOR: Rgb565 = rgb565(155, 155, 155);
pub const FISH_TAIL_COLOR: Rgb565 = rgb565(30, 30, 30);
pub const LEAF_COLOR: Rgb565 = rgb565(62, 145, 60);
pub const DUCKWEED_COLOR: Rgb565 = rgb565(93, 206, 71);
pub const RIG_COLOR: Rgb565 = rgb565(36, 109, 243);

pub fn create_fishes<const N: usize>(
    number: usize,
    width: f32,
    height: f32,
    spatial_scale: f32,
    now_ms: u64,
    rng: &mut Random,
) -> Vec<Fish, N> {
    let mut fishes = Vec::new();
    for _ in 0..number.min(N) {
        let fish_size = sqrt(width * width + height * height) * 0.015 * rng.range(0.8, 1.2);
        let x = rng.range(0.0, width);
        let y = rng.range(0.0, height);
        let length = fish_size * rng.range(6.0, 8.5);
        let _ = fishes.push(Fish::new(
            rng.sign() * 2.0 * x * rng.range(0.8, 1.2),
            rng.sign() * 2.0 * y * rng.range(0.8, 1.2),
            fish_size * rng.range(6.0, 8.5),
            length * rng.range(0.2, 0.24),
            FishScene {
                viewport_width: width,
                spatial_scale,
            },
            now_ms,
            rng,
        ));
    }
    fishes
}

pub fn create_leaves<const N: usize>(
    number: usize,
    width: f32,
    height: f32,
    rng: &mut Random,
) -> Vec<Leaf, N> {
    let mut leaves = Vec::new();
    let size = sqrt(width * width + height * height);
    for _ in 0..number.min(N) {
        let _ = leaves.push(Leaf::new(
            rng.range(0.0, width),
            rng.range(0.0, height),
            rng.range(size * 0.02, size * 0.05),
            64,
            rng,
        ));
    }
    leaves
}

pub fn create_duckweeds<const N: usize>(
    number: usize,
    width: f32,
    height: f32,
    rng: &mut Random,
) -> Vec<DuckWeed, N> {
    let mut weeds = Vec::new();
    let size = sqrt(width * width + height * height);
    for _ in 0..number.min(N) {
        let _ = weeds.push(DuckWeed::new(
            rng.range(0.0, width),
            rng.range(0.0, height),
            rng.range(size * 0.001, size * 0.005),
            4,
            rng,
        ));
    }
    weeds
}
