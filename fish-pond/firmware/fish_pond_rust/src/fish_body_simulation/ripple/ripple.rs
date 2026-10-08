use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::Rgb565,
    prelude::Primitive,
    primitives::{Circle, PrimitiveStyle},
};
use heapless::Vec;

use crate::fish_body_simulation::functions::{lerp, map, rgb565};

#[derive(Clone, Copy)]
pub struct Edges {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Clone, Copy)]
pub struct RippleData {
    pub initial_intensity: f32,
    pub current_intensity: f32,
    pub target_radius: f32,
    pub current_radius: f32,
    pub edges: Edges,
}

pub struct Ripple {
    pub max_intensity: f32,
    pub current_intensity: f32,
    pub interval_ms: u64,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub spatial_scale: f32,
    pub start_ms: u64,
    pub ripple_group: Vec<RippleData, 4>,
    pub remaining_ripples: u8,
}

impl Ripple {
    pub fn new(
        x: f32,
        y: f32,
        intensity: f32,
        radius: f32,
        now_ms: u64,
        spatial_scale: f32,
    ) -> Self {
        let mut ripple_group = Vec::new();
        let _ = ripple_group.push(RippleData {
            initial_intensity: intensity,
            current_intensity: intensity,
            target_radius: radius,
            current_radius: radius,
            edges: Edges {
                left: x,
                right: x,
                top: y,
                bottom: y,
            },
        });
        Self {
            max_intensity: intensity,
            current_intensity: intensity,
            interval_ms: 150,
            x,
            y,
            speed: intensity / 100.0,
            spatial_scale,
            start_ms: now_ms,
            ripple_group,
            remaining_ripples: map(intensity, 0.0, 255.0, 0.0, 3.0) as u8,
        }
    }

    pub fn update(&mut self, now_ms: u64) {
        if self.remaining_ripples > 0 && now_ms.saturating_sub(self.start_ms) > self.interval_ms {
            let _ = self.ripple_group.push(RippleData {
                initial_intensity: self.current_intensity,
                current_intensity: self.current_intensity,
                current_radius: 0.0,
                target_radius: 0.0,
                edges: Edges {
                    left: self.x,
                    right: self.x,
                    top: self.y,
                    bottom: self.y,
                },
            });
            self.start_ms = now_ms;
            self.remaining_ripples -= 1;
        }
        self.current_intensity -= self.speed;
        for ripple in &mut self.ripple_group {
            ripple.current_intensity -= self.speed;
            ripple.target_radius += self.speed * 5.0 * self.spatial_scale;
        }
        let mut index = self.ripple_group.len();
        while index > 0 {
            index -= 1;
            if self.ripple_group[index].current_intensity <= 0.0 {
                self.ripple_group.swap_remove(index);
            }
        }
    }

    pub fn advance_visual(&mut self) {
        for ripple in &mut self.ripple_group {
            ripple.current_radius = lerp(ripple.current_radius, ripple.target_radius, 0.1);
        }
    }

    pub fn draw<D>(&self, target: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        for ripple in &self.ripple_group {
            let intensity =
                map(ripple.current_intensity, 0.0, 255.0, 0.0, 255.0).clamp(0.0, 255.0) as u8;
            let diameter = (ripple.current_radius * 2.0).max(1.0) as u32;
            let top_left = Point::new(
                (self.x - ripple.current_radius) as i32,
                (self.y - ripple.current_radius) as i32,
            );
            let color = rgb565(intensity, intensity, intensity);
            let stroke_width = libm::roundf(self.spatial_scale).max(1.0) as u32;
            let _ = Circle::new(top_left, diameter)
                .into_styled(PrimitiveStyle::with_stroke(color, stroke_width))
                .draw(target);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.ripple_group.is_empty()
    }

    pub fn detect_bouncing(&mut self, width: f32, height: f32, now_ms: u64) -> Vec<Ripple, 4> {
        let mut result = Vec::new();
        for ripple in &mut self.ripple_group {
            let current_left = self.x - ripple.current_radius;
            let current_right = self.x + ripple.current_radius;
            let current_top = self.y - ripple.current_radius;
            let current_bottom = self.y + ripple.current_radius;
            let intensity = ripple.current_intensity * 0.6;
            if ripple.edges.left > 0.0 && current_left <= 0.0 {
                let _ = result.push(Ripple::new(
                    -self.x,
                    self.y,
                    intensity,
                    ripple.current_radius,
                    now_ms,
                    self.spatial_scale,
                ));
            }
            if ripple.edges.right < width && current_right >= width {
                let _ = result.push(Ripple::new(
                    width * 2.0 - self.x,
                    self.y,
                    intensity,
                    ripple.current_radius,
                    now_ms,
                    self.spatial_scale,
                ));
            }
            if ripple.edges.top > 0.0 && current_top <= 0.0 {
                let _ = result.push(Ripple::new(
                    self.x,
                    -self.y,
                    intensity,
                    ripple.current_radius,
                    now_ms,
                    self.spatial_scale,
                ));
            }
            if ripple.edges.bottom < height && current_bottom >= height {
                let _ = result.push(Ripple::new(
                    self.x,
                    height * 2.0 - self.y,
                    intensity,
                    ripple.current_radius,
                    now_ms,
                    self.spatial_scale,
                ));
            }
            ripple.edges = Edges {
                left: current_left,
                right: current_right,
                top: current_top,
                bottom: current_bottom,
            };
        }
        result
    }
}
