use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};
use heapless::Vec;

use crate::fish_body_simulation::{
    fish::fish::Fish,
    functions::{Point, Random, random_point_outside_rect},
    leaf::{duckweed::DuckWeed, leaf::Leaf},
    ripple::ripple::Ripple,
    sketch::{
        sketch_init::{
            BACKGROUND_COLOR, DUCKWEED_COLOR, EYE_COLOR, FISH_COLOR, FISH_FIN_COLOR,
            FISH_OUTLINE_COLOR, FISH_TAIL_COLOR, LEAF_COLOR, RIG_COLOR, create_duckweeds,
            create_fishes, create_leaves,
        },
        sketch_update::{
            detect_fish_duckweed_collision, detect_fish_leaf_collision,
            detect_ripple_duckweed_collision, detect_ripple_leaf_collision, is_overlapping,
            update_plants,
        },
    },
};

pub const MAX_FISHES: usize = 12;
pub const MAX_LEAVES: usize = 32;
pub const MAX_DUCKWEEDS: usize = 100;
pub const MAX_RIPPLES: usize = 64;
const REFERENCE_MIN_DIMENSION: f32 = 640.0;
const FIXED_UPDATE_MS: u32 = 1000 / 60;
const MAX_FIXED_UPDATES_PER_STEP: u32 = 4;

#[derive(Clone, Copy, Debug)]
pub struct SceneConfig {
    pub width: u32,
    pub height: u32,
    pub fish_count: usize,
    pub leaf_count: usize,
    pub duckweed_count: usize,
    pub seed: u64,
}

impl SceneConfig {
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            fish_count: 5,
            leaf_count: 40,
            duckweed_count: 10,
            seed: 0x6d2b_79f5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChaseDirection {
    BottomLeft,
    BottomRight,
    TopCenter,
}

pub struct Pond {
    pub width: f32,
    pub height: f32,
    pub spatial_scale: f32,
    pub fishes: Vec<Fish, MAX_FISHES>,
    pub ripples: Vec<Ripple, MAX_RIPPLES>,
    pub leaves: Vec<Leaf, MAX_LEAVES>,
    pub duckweeds: Vec<DuckWeed, MAX_DUCKWEEDS>,
    pub show_rig: bool,
    pub chase_direction: Option<ChaseDirection>,
    rng: Random,
    now_ms: u64,
    fixed_accumulator_ms: u32,
    ripple_cooldown_ms: u32,
    last_ripple_time_ms: u64,
}

impl Pond {
    pub fn new(config: SceneConfig) -> Self {
        let mut rng = Random::new(config.seed);
        let width = config.width as f32;
        let height = config.height as f32;
        let spatial_scale = width.min(height).max(1.0) / REFERENCE_MIN_DIMENSION;
        let fishes = create_fishes(config.fish_count, width, height, spatial_scale, 0, &mut rng);
        let leaves = create_leaves(config.leaf_count, width, height, &mut rng);
        let duckweeds = create_duckweeds(config.duckweed_count, width, height, &mut rng);
        let ripple_cooldown_ms = rng.range(500.0, 1500.0) as u32;
        Self {
            width,
            height,
            spatial_scale,
            fishes,
            ripples: Vec::new(),
            leaves,
            duckweeds,
            show_rig: false,
            chase_direction: None,
            rng,
            now_ms: 0,
            fixed_accumulator_ms: 0,
            ripple_cooldown_ms,
            last_ripple_time_ms: 0,
        }
    }

    /// Advances and draws exactly one caller-controlled frame.
    ///
    /// `elapsed_ms` replaces browser timers and requestAnimationFrame. The
    /// caller can sleep, schedule, or skip calls as required by its platform.
    pub fn step<D>(&mut self, target: &mut D, elapsed_ms: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        self.update(elapsed_ms);
        self.render(target);
    }

    /// Advances simulation state without drawing.
    ///
    /// Fixed-step catch-up is bounded to avoid a slow display frame causing
    /// hundreds of hidden updates before the next visible frame.
    pub fn update(&mut self, elapsed_ms: u32) {
        self.now_ms = self.now_ms.saturating_add(elapsed_ms as u64);
        self.fixed_accumulator_ms = self.fixed_accumulator_ms.saturating_add(elapsed_ms);

        if self.now_ms.saturating_sub(self.last_ripple_time_ms) >= self.ripple_cooldown_ms as u64 {
            let point = Point {
                x: self.rng.range(0.0, self.width),
                y: self.rng.range(0.0, self.height),
            };
            self.add_ripple(point.x, point.y, 100.0);
            self.ripple_cooldown_ms = self.rng.range(2000.0, 7000.0) as u32;
            self.last_ripple_time_ms = self.now_ms;
        }

        let mut updates = 0;
        while self.fixed_accumulator_ms >= FIXED_UPDATE_MS && updates < MAX_FIXED_UPDATES_PER_STEP {
            self.fixed_update();
            self.fixed_accumulator_ms -= FIXED_UPDATE_MS;
            updates += 1;
        }
        if self.fixed_accumulator_ms >= FIXED_UPDATE_MS {
            self.fixed_accumulator_ms %= FIXED_UPDATE_MS;
        }
        for fish in &mut self.fishes {
            fish.move_body(self.width, self.height);
        }
        for ripple in &mut self.ripples {
            ripple.advance_visual();
        }
        for weed in &mut self.duckweeds {
            weed.advance_visual();
        }
        for leaf in &mut self.leaves {
            leaf.advance_visual();
        }
    }

    fn fixed_update(&mut self) {
        for fish in &mut self.fishes {
            if let Some(direction) = self.chase_direction {
                let target = match direction {
                    ChaseDirection::BottomLeft => Point {
                        x: 0.0,
                        y: self.height,
                    },
                    ChaseDirection::BottomRight => Point {
                        x: self.width,
                        y: self.height,
                    },
                    ChaseDirection::TopCenter => Point {
                        x: self.width / 2.0,
                        y: 0.0,
                    },
                };
                fish.cube.chase(target.x, target.y);
            }
            fish.update(self.width, self.height, &mut self.rng);
            detect_fish_leaf_collision(fish, self.leaves.as_mut_slice());
            detect_fish_duckweed_collision(fish, self.duckweeds.as_mut_slice());
        }

        for index in 0..self.fishes.len() {
            if !self.fishes[index].is_dashing() {
                continue;
            }
            let bounds = self.fishes[index].bounds;
            for other in 0..self.fishes.len() {
                if index != other && is_overlapping(bounds, self.fishes[other].bounds) {
                    self.fishes[other].trigger_dash();
                }
            }
            if self
                .now_ms
                .saturating_sub(self.fishes[index].last_ripple_time_ms)
                > self.fishes[index].ripple_cooldown_ms as u64
            {
                let ripple = self.fishes[index].create_ripple(
                    self.now_ms,
                    self.spatial_scale,
                    &mut self.rng,
                );
                let _ = self.ripples.push(ripple);
                self.fishes[index].last_ripple_time_ms = self.now_ms;
            }
        }

        let mut index = self.fishes.len();
        while index > 0 {
            index -= 1;
            if self.fishes[index].end_leave {
                self.fishes.swap_remove(index);
            }
        }

        let original_ripples = self.ripples.len();
        for index in 0..original_ripples {
            detect_ripple_leaf_collision(&self.ripples[index], self.leaves.as_mut_slice());
            detect_ripple_duckweed_collision(&self.ripples[index], self.duckweeds.as_mut_slice());
            let reflections =
                self.ripples[index].detect_bouncing(self.width, self.height, self.now_ms);
            for ripple in reflections {
                let _ = self.ripples.push(ripple);
            }
        }
        update_plants(
            self.leaves.as_mut_slice(),
            self.duckweeds.as_mut_slice(),
            self.width,
            self.height,
            &mut self.rng,
        );
        for ripple in &mut self.ripples {
            ripple.update(self.now_ms);
        }
        let mut index = self.ripples.len();
        while index > 0 {
            index -= 1;
            if self.ripples[index].is_empty() {
                self.ripples.swap_remove(index);
            }
        }
    }

    pub fn render<D>(&self, target: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let _ = target.clear(BACKGROUND_COLOR);
        let body_stroke = Self::scaled_stroke(2, self.spatial_scale);
        let detail_stroke = Self::scaled_stroke(1, self.spatial_scale);
        for fish in &self.fishes {
            fish.draw_fins(target, FISH_FIN_COLOR, FISH_OUTLINE_COLOR, body_stroke);
            fish.draw_tail(target, FISH_TAIL_COLOR, FISH_OUTLINE_COLOR, body_stroke);
            fish.draw_body(target, FISH_COLOR, FISH_OUTLINE_COLOR, body_stroke);
            if fish.show_rig {
                fish.draw_rig(target, RIG_COLOR);
            }
            fish.draw_back_fin(target, FISH_FIN_COLOR, FISH_OUTLINE_COLOR, detail_stroke);
            fish.draw_eyes(target, FISH_OUTLINE_COLOR, EYE_COLOR);
        }
        for ripple in &self.ripples {
            ripple.draw(target);
        }
        for weed in &self.duckweeds {
            weed.draw(target, DUCKWEED_COLOR, BACKGROUND_COLOR, detail_stroke);
        }
        for leaf in &self.leaves {
            leaf.draw(target, LEAF_COLOR, BACKGROUND_COLOR, body_stroke);
        }
    }

    pub fn add_ripple(&mut self, x: f32, y: f32, intensity: f32) {
        let _ = self.ripples.push(Ripple::new(
            x,
            y,
            intensity,
            0.0,
            self.now_ms,
            self.spatial_scale,
        ));
    }

    fn scaled_stroke(base: u32, spatial_scale: f32) -> u32 {
        libm::roundf(base as f32 * spatial_scale).max(1.0) as u32
    }

    pub fn set_show_rig(&mut self, show: bool) {
        self.show_rig = show;
        for fish in &mut self.fishes {
            fish.show_rig = show;
        }
    }

    pub fn dash_at(&mut self, x: f32, y: f32) {
        for fish in &mut self.fishes {
            let bounds = fish.bounds;
            if x >= bounds.left && x <= bounds.right && y >= bounds.top && y <= bounds.bottom {
                fish.trigger_directional_dash(x, y);
            }
        }
    }

    pub fn remove_fish(&mut self, index: usize) {
        if let Some(fish) = self.fishes.get_mut(index) {
            fish.leave_target = random_point_outside_rect(self.width, self.height, &mut self.rng);
            fish.on_leave = true;
        }
    }
}
