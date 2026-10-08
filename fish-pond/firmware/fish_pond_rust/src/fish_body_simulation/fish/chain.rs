use core::f32::consts::PI;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};
use heapless::Vec;

use super::circle::Circle;
use crate::fish_body_simulation::functions::{
    Point, draw_circle, draw_polygon, find_angle_between, find_tangent, is_on_left, line, map,
};

pub const MAX_CHAIN: usize = 16;

#[derive(Clone)]
pub struct Chain {
    pub circles: Vec<Circle, MAX_CHAIN>,
    pub gap: f32,
    pub smallest_angle: f32,
    pub frame_count: f32,
}

impl Chain {
    pub fn new(x: f32, y: f32, gap: f32, angle: f32, sizes: &[f32]) -> Self {
        let mut circles = Vec::new();
        let mut cursor_x = x;
        for size in sizes.iter().take(MAX_CHAIN) {
            let _ = circles.push(Circle::new(cursor_x, y, *size));
            cursor_x += gap;
        }
        Self {
            circles,
            gap,
            smallest_angle: angle * PI / 180.0,
            frame_count: 0.0,
        }
    }

    pub fn free_move(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let acceleration = self.circles[0].follow_mouse(x, y, width, height);
        self.frame_count += 25.0 * libm::logf(0.3 * acceleration + 1.0);
        let scale = (PI / 5.0) * libm::logf(2.0 * acceleration + 1.0);
        let length = self.circles.len();
        for i in 1..length {
            let offset = i as f32 * length as f32 * PI * 1.1368;
            let oscillation = libm::sinf(self.frame_count + offset)
                * scale
                * map(i as f32, 0.0, length as f32, 0.5, 3.0);
            let target = self.circles[i - 1];
            let other = i.checked_sub(2).map(|j| self.circles[j]);
            self.circles[i].follow_body(
                target,
                other,
                self.gap,
                self.smallest_angle,
                Some(oscillation),
            );
        }
    }

    pub fn constrain_move(&mut self, x: f32, y: f32, ideal_radian: f32, strength: f32) {
        self.circles[0].teleport(x, y);
        if self.circles.len() < 2 {
            return;
        }
        let ideal = Point {
            x: x + self.gap * libm::cosf(ideal_radian),
            y: y + self.gap * libm::sinf(ideal_radian),
        };
        let center = Point { x, y };
        let current = self.circles[1].position();
        let delta = find_angle_between(center, current, ideal);
        let current_radian = find_tangent(center, current);
        let direction = if is_on_left(center, ideal, current) {
            -1.0
        } else {
            1.0
        };
        let radian = current_radian + direction * delta * strength;
        self.circles[1].teleport(
            x + self.gap * libm::cosf(radian),
            y + self.gap * libm::sinf(radian),
        );
        self.follow_from(2);
    }

    pub fn simple_move(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.circles[0].follow_mouse(x, y, width, height);
        self.follow_from(1);
    }

    fn follow_from(&mut self, start: usize) {
        for i in start..self.circles.len() {
            let target = self.circles[i - 1];
            let other = i.checked_sub(2).map(|j| self.circles[j]);
            self.circles[i].follow_body(target, other, self.gap, self.smallest_angle, None);
        }
    }

    pub fn draw_outline<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565, stroke_width: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        if self.circles.len() < 2 {
            return;
        }
        let mut points: Vec<Point, 34> = Vec::new();
        for i in 0..self.circles.len() {
            let radian = self.side_radian(i, true);
            if i == 0 {
                let head =
                    find_tangent(self.circles[0].position(), self.circles[1].position()) - 0.5 * PI;
                let _ = points.push(calculate_point(self.circles[0], head));
            }
            let _ = points.push(calculate_point(self.circles[i], radian));
        }
        for i in (0..self.circles.len()).rev() {
            let _ = points.push(calculate_point(self.circles[i], self.side_radian(i, false)));
        }
        draw_polygon(target, &points, fill, stroke, stroke_width);
    }

    fn side_radian(&self, i: usize, left: bool) -> f32 {
        let last = self.circles.len() - 1;
        if i == 0 {
            return find_tangent(self.circles[i].position(), self.circles[i + 1].position())
                + if left { 0.5 * PI } else { -0.5 * PI };
        }
        if i == last {
            return find_tangent(self.circles[i].position(), self.circles[i - 1].position())
                + if left { -0.5 * PI } else { 0.5 * PI };
        }
        let (next, previous) = if left { (i + 1, i - 1) } else { (i - 1, i + 1) };
        let delta = find_angle_between(
            self.circles[i].position(),
            self.circles[next].position(),
            self.circles[previous].position(),
        );
        let alpha = find_tangent(
            self.circles[i].position(),
            self.circles[previous].position(),
        );
        if is_on_left(
            self.circles[i].position(),
            self.circles[next].position(),
            self.circles[previous].position(),
        ) {
            alpha - delta / 2.0
        } else {
            alpha - (2.0 * PI - delta) / 2.0
        }
    }

    pub fn draw_rig<D>(&self, target: &mut D, color: Rgb565)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        for i in 1..self.circles.len().saturating_sub(1) {
            let circle = self.circles[i];
            draw_circle(target, circle.x, circle.y, circle.d, color);
            line(
                target,
                circle.position(),
                self.circles[i + 1].position(),
                color,
            );
        }
    }
}

fn calculate_point(circle: Circle, radian: f32) -> Point {
    Point {
        x: circle.x + circle.d / 2.0 * libm::cosf(radian),
        y: circle.y + circle.d / 2.0 * libm::sinf(radian),
    }
}
