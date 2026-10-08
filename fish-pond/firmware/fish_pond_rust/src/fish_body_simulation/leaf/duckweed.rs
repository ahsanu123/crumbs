use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};
use heapless::Vec;

use crate::fish_body_simulation::{
    functions::{PI2, Point, Random, draw_polygon, find_position, normalize_vector, sqrt},
    leaf::leaf::LeafPoint,
};

#[derive(Clone, Copy)]
pub struct Axis {
    pub current: f32,
    pub target: f32,
}

pub struct DuckWeed {
    pub radius: f32,
    pub x: Axis,
    pub y: Axis,
    pub points: Vec<LeafPoint, 4>,
    pub move_vector: Point,
    pub vector_max: f32,
    pub longest_radius: f32,
}

impl DuckWeed {
    pub fn new(x: f32, y: f32, radius: f32, segments: usize, rng: &mut Random) -> Self {
        let first = rng.range(0.0, PI2);
        let segment_radian = PI2 / segments as f32;
        let mut points = Vec::new();
        let mut longest_radius: f32 = 0.0;
        for index in 0..segments.min(4) {
            let length = rng.range(radius * 0.98, radius * 1.02);
            longest_radius = longest_radius.max(length);
            let _ = points.push(LeafPoint {
                length,
                radian: first + segment_radian * index as f32,
            });
        }
        Self {
            radius,
            x: Axis {
                current: x,
                target: x,
            },
            y: Axis {
                current: y,
                target: y,
            },
            points,
            move_vector: Point::default(),
            vector_max: radius * 0.1,
            longest_radius,
        }
    }

    pub fn update(&mut self, width: f32, height: f32, rng: &mut Random) {
        self.x.target += self.move_vector.x;
        self.y.target += self.move_vector.y;
        self.move_vector.x *= 0.99;
        self.move_vector.y *= 0.99;
        if self.x.current + self.longest_radius < 0.0
            || self.x.current - self.longest_radius > width
            || self.y.current + self.longest_radius < 0.0
            || self.y.current - self.longest_radius > height
        {
            self.x.current = rng.range(0.0, width);
            self.x.target = self.x.current;
            self.y.current = rng.range(0.0, height);
            self.y.target = self.y.current;
        }
    }

    pub fn advance_visual(&mut self) {
        self.x.current += (self.x.target - self.x.current) * 0.1;
        self.y.current += (self.y.target - self.y.current) * 0.1;
    }

    pub fn apply_vector(&mut self, x: f32, y: f32, strength: f32) {
        let new = normalize_vector(
            Point {
                x: self.x.current - x,
                y: self.y.current - y,
            },
            strength,
        );
        let mut result = Point {
            x: new.x + self.move_vector.x,
            y: new.y + self.move_vector.y,
        };
        if sqrt(result.x * result.x + result.y * result.y) > self.vector_max {
            result = normalize_vector(result, self.vector_max);
        }
        self.move_vector = result;
    }

    pub fn draw<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565, stroke_width: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let mut points: Vec<Point, 4> = Vec::new();
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
