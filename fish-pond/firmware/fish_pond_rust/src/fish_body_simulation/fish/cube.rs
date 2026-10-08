use super::chain::Chain;
use crate::fish_body_simulation::functions::{Point, Random, sqrt};

pub struct Cube {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub chain: Chain,
    pub direction_x: f32,
    pub direction_y: f32,
    pub v_max: f32,
    pub v_min: f32,
    pub v_dash: f32,
    pub w: f32,
    pub h: f32,
    pub p_boost: f32,
    pub p_direction_change: f32,
}

impl Cube {
    pub fn new(x: f32, y: f32, v_max: f32, viewport_width: f32, rng: &mut Random) -> Self {
        Self {
            x,
            y,
            vx: rng.range(0.0, v_max),
            vy: rng.range(0.0, v_max),
            chain: Chain::new(x, y, v_max, 160.0, &[0.0; 12]),
            direction_x: rng.sign(),
            direction_y: rng.sign(),
            v_max,
            v_min: v_max * 0.1,
            v_dash: v_max * 2.0,
            w: viewport_width * 0.01,
            h: viewport_width * 0.01,
            p_boost: 0.005,
            p_direction_change: 0.001,
        }
    }

    pub fn update(&mut self, width: f32, height: f32, check_bound: bool, rng: &mut Random) {
        if rng.unit() < self.p_boost || self.vx - self.v_min <= 0.002 {
            self.vx = rng.range(self.v_max / 2.0, self.v_max);
            if rng.unit() < 0.2 {
                self.direction_x *= -1.0;
            }
        }
        if rng.unit() < self.p_boost || self.vy - self.v_min <= 0.002 {
            self.vy = rng.range(self.v_max / 2.0, self.v_max);
            if rng.unit() < 0.2 {
                self.direction_y *= -1.0;
            }
        }
        if self.vx > self.v_min {
            self.vx -= (self.vx - self.v_min) * rng.range(0.01, 0.02);
        }
        if self.vy > self.v_min {
            self.vy -= (self.vy - self.v_min) * rng.range(0.01, 0.02);
        }
        if rng.unit() < self.p_direction_change {
            self.direction_x *= -1.0;
        }
        if rng.unit() < self.p_direction_change {
            self.direction_y *= -1.0;
        }
        self.x += self.vx * self.direction_x;
        self.y += self.vy * self.direction_y;
        if check_bound {
            self.prevent_over_border(width, height);
        }
        self.chain.simple_move(self.x, self.y, width, height);
    }

    fn prevent_over_border(&mut self, width: f32, height: f32) {
        if self.x + self.w / 2.0 >= width {
            self.x = width - self.w / 2.0;
            self.direction_x *= -1.0;
        } else if self.x - self.w / 2.0 <= 0.0 {
            self.x = self.w / 2.0;
            self.direction_x *= -1.0;
        } else if self.y + self.h / 2.0 >= height {
            self.y = height - self.h / 2.0;
            self.direction_y *= -1.0;
        } else if self.y - self.h / 2.0 <= 0.0 {
            self.y = self.h / 2.0;
            self.direction_y *= -1.0;
        }
    }

    pub fn dash(&mut self, radian: f32) {
        let vx = self.v_dash * libm::cosf(radian);
        let vy = self.v_dash * libm::sinf(radian);
        self.vx = libm::fabsf(vx);
        self.vy = libm::fabsf(vy);
        self.direction_x = if vx >= 0.0 { 1.0 } else { -1.0 };
        self.direction_y = if vy >= 0.0 { 1.0 } else { -1.0 };
    }

    pub fn chase(&mut self, x: f32, y: f32) {
        let mut dx = x - self.x;
        let mut dy = y - self.y;
        let magnitude = sqrt(dx * dx + dy * dy);
        if magnitude < 1.0 {
            return;
        }
        if magnitude > self.v_max {
            dx *= self.v_max / magnitude * 1.2;
            dy *= self.v_max / magnitude * 1.2;
        }
        self.vx += libm::fabsf(dx) * 0.01;
        self.direction_x = if self.x < x { 1.0 } else { -1.0 };
        self.vy += libm::fabsf(dy) * 0.01;
        self.direction_y = if self.y < y { 1.0 } else { -1.0 };
    }

    pub fn position(&self) -> Point {
        self.chain.circles[self.chain.circles.len() - 1].position()
    }
}
