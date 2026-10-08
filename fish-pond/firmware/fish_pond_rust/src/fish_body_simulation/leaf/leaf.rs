use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};
use heapless::Vec;

use crate::fish_body_simulation::functions::{
    PI2, Point, Random, draw_polygon, find_position, normalize_vector, sqrt,
};

#[derive(Clone, Copy)]
pub struct Axis {
    pub original: f32,
    pub current: f32,
    pub target: f32,
}

#[derive(Clone, Copy)]
pub struct LeafPoint {
    pub length: f32,
    pub radian: f32,
}

pub struct Leaf {
    pub radius: f32,
    pub x: Axis,
    pub y: Axis,
    pub points: Vec<LeafPoint, 64>,
    pub frame_count: f32,
    pub oscillate_vector: Point,
    pub oscillate_max: f32,
}

impl Leaf {
    pub fn new(x: f32, y: f32, radius: f32, segments: usize, rng: &mut Random) -> Self {
        let first = rng.range(0.0, PI2);
        let segment_radian = PI2 / segments as f32;
        let mut points = Vec::new();
        for index in 0..segments.min(64) {
            let length = if index == 0 {
                rng.range(radius * 0.1, radius * 0.2)
            } else {
                rng.range(radius * 0.98, radius * 1.02)
            };
            let _ = points.push(LeafPoint {
                length,
                radian: first + segment_radian * index as f32,
            });
        }
        Self {
            radius,
            x: Axis {
                original: x,
                current: x,
                target: x,
            },
            y: Axis {
                original: y,
                current: y,
                target: y,
            },
            points,
            frame_count: 0.0,
            oscillate_vector: Point::default(),
            oscillate_max: radius * 0.4,
        }
    }

    pub fn update(&mut self) {
        let acceleration = sqrt(
            self.oscillate_vector.x * self.oscillate_vector.x
                + self.oscillate_vector.y * self.oscillate_vector.y,
        );
        self.frame_count += 0.1 * libm::logf(0.01 * acceleration + 1.0);
        self.x.target = self.x.original + libm::sinf(self.frame_count) * self.oscillate_vector.x;
        self.y.target = self.y.original + libm::sinf(self.frame_count) * self.oscillate_vector.y;
        self.oscillate_vector.x *= 0.99;
        self.oscillate_vector.y *= 0.99;
    }

    pub fn advance_visual(&mut self) {
        self.x.current += (self.x.target - self.x.current) * 0.1;
        self.y.current += (self.y.target - self.y.current) * 0.1;
    }

    pub fn apply_oscillation(&mut self, x: f32, y: f32, strength: f32) {
        let new = normalize_vector(
            Point {
                x: self.x.current - x,
                y: self.y.current - y,
            },
            strength,
        );
        let mut result = Point {
            x: new.x + self.oscillate_vector.x,
            y: new.y + self.oscillate_vector.y,
        };
        if sqrt(result.x * result.x + result.y * result.y) > self.oscillate_max {
            result = normalize_vector(result, self.oscillate_max);
        }
        self.oscillate_vector = result;
    }

    pub fn draw<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565, stroke_width: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let mut points: Vec<Point, 64> = Vec::new();
        for point in &self.points {
            let position = find_position(
                Point {
                    x: self.x.current,
                    y: self.y.current,
                },
                point.radian,
                point.length,
            );
            let _ = points.push(position);
        }
        draw_polygon(target, &points, fill, stroke, stroke_width);
    }

    pub fn position(&self) -> Point {
        Point {
            x: self.x.current,
            y: self.y.current,
        }
    }
}
