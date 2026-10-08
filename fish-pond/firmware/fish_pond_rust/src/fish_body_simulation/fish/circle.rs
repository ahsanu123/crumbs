use core::f32::consts::PI;

use crate::fish_body_simulation::functions::{
    Point, dist, find_angle_between, find_tangent, is_on_left, lerp, map, sqrt,
};

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub d: f32,
}

impl Circle {
    pub const fn new(x: f32, y: f32, d: f32) -> Self {
        Self { x, y, d }
    }

    pub fn follow_mouse(&mut self, mouse_x: f32, mouse_y: f32, width: f32, height: f32) -> f32 {
        let x = lerp(self.x, mouse_x, 0.1);
        let y = lerp(self.y, mouse_y, 0.1);
        let radian = find_tangent(self.position(), Point { x, y }) + 0.5 * PI;
        let factor = map(
            dist(self.x, self.y, mouse_x, mouse_y),
            0.0,
            sqrt(width * width + height * height),
            0.0,
            1.0,
        );
        let displace_x = libm::cosf(radian) * factor;
        let displace_y = libm::sinf(radian) * factor;
        self.x = x + displace_x;
        self.y = y + displace_y;
        sqrt(displace_x * displace_x + displace_y * displace_y)
    }

    pub fn follow_body(
        &mut self,
        target: Circle,
        target_of_target: Option<Circle>,
        gap: f32,
        smallest_angle: f32,
        oscillate_radian: Option<f32>,
    ) {
        self.apply_pulling_force(target, gap, oscillate_radian);
        if let Some(other) = target_of_target {
            self.apply_angle_constraint(target.position(), other.position(), gap, smallest_angle);
        }
    }

    pub fn teleport(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn apply_pulling_force(&mut self, target: Circle, gap: f32, oscillate: Option<f32>) {
        let radian = find_tangent(target.position(), self.position()) + oscillate.unwrap_or(0.0);
        self.x = target.x + gap * libm::cosf(radian);
        self.y = target.y + gap * libm::sinf(radian);
    }

    fn apply_angle_constraint(&mut self, center: Point, other: Point, gap: f32, minimum: f32) {
        let delta = find_angle_between(center, self.position(), other);
        if delta < minimum {
            let other_radian = find_tangent(center, other);
            let radian = if is_on_left(center, other, self.position()) {
                other_radian + minimum
            } else {
                other_radian - minimum
            };
            self.x = center.x + gap * libm::cosf(radian);
            self.y = center.y + gap * libm::sinf(radian);
        }
        if delta != PI {
            let ideal = find_tangent(other, center) + PI;
            let current = find_tangent(center, self.position());
            let difference = libm::fabsf(ideal - current);
            let radian = if is_on_left(center, other, self.position()) {
                current + difference * 0.001
            } else {
                current - difference * 0.001
            };
            self.x = center.x + gap * libm::cosf(radian);
            self.y = center.y + gap * libm::sinf(radian);
        }
    }

    pub const fn position(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}
