use core::f32::consts::PI;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};
use heapless::Vec;

use super::{chain::Chain, cube::Cube};
use crate::fish_body_simulation::{
    functions::{Point, Random, draw_filled_circle, draw_polygon, find_tangent, map, sqrt},
    ripple::ripple::Ripple,
};

const BODY_POINTS: [f32; 14] = [
    0.326, 0.641, 0.817, 0.9, 0.97, 0.957, 0.872, 0.787, 0.702, 0.618, 0.516, 0.414, 0.316, 0.219,
];
const FIN_POINTS: [f32; 7] = [0.226, 0.217, 0.334, 0.476, 0.424, 0.355, 0.11];
const TAIL_POINTS: [f32; 8] = [0.326, 0.321, 0.32, 0.294, 0.283, 0.216, 0.155, 0.09];
const BACK_FIN_POINTS: [f32; 3] = [0.5, 0.5, 0.5];

pub struct AttachedChain {
    pub chain: Chain,
    pub radian: f32,
    pub position: usize,
}

#[derive(Clone, Copy)]
pub struct FishScene {
    pub viewport_width: f32,
    pub spatial_scale: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Bounds {
    pub fn x(&self) -> f32 {
        self.left
    }
    pub fn y(&self) -> f32 {
        self.top
    }
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }
    pub fn center(&self) -> Point {
        Point {
            x: self.left + self.width() / 2.0,
            y: self.top + self.height() / 2.0,
        }
    }
}

pub struct Fish {
    pub gap: f32,
    pub body: Chain,
    pub fins: Vec<AttachedChain, 4>,
    pub tails: Vec<AttachedChain, 2>,
    pub back_fins: Vec<AttachedChain, 1>,
    pub bounds: Bounds,
    pub cube: Cube,
    pub ripple_cooldown_ms: u32,
    pub last_ripple_time_ms: u64,
    pub show_rig: bool,
    pub on_leave: bool,
    pub end_leave: bool,
    pub leave_target: Point,
}

impl Fish {
    pub fn new(
        x: f32,
        y: f32,
        length: f32,
        width: f32,
        scene: FishScene,
        now_ms: u64,
        rng: &mut Random,
    ) -> Self {
        let gap = length / BODY_POINTS.len() as f32;
        let mut body_sizes = [0.0; BODY_POINTS.len()];
        for (out, point) in body_sizes.iter_mut().zip(BODY_POINTS) {
            *out = point * width;
        }
        let body = Chain::new(x, y, gap, 165.0, &body_sizes);

        let mut fins = Vec::new();
        for (index, position) in [2_usize, 2, 6, 6].into_iter().enumerate() {
            let factor = BODY_POINTS[position] * 0.8;
            let mut sizes = [0.0; FIN_POINTS.len()];
            for (out, point) in sizes.iter_mut().zip(FIN_POINTS) {
                *out = point * width * if index <= 1 { 1.5 } else { 1.0 } * factor;
            }
            let _ = fins.push(AttachedChain {
                chain: Chain::new(
                    x,
                    y,
                    gap * 2.5 * (width / length),
                    if index <= 1 { 175.0 } else { 155.0 } + 20.0 * (width / length),
                    &sizes,
                ),
                radian: PI / 1.8 * if index % 2 == 0 { 1.0 } else { -1.0 } * factor,
                position,
            });
        }

        let mut tails = Vec::new();
        for (index, position) in [12_usize, 12].into_iter().enumerate() {
            let mut sizes = [0.0; TAIL_POINTS.len()];
            for (out, point) in sizes.iter_mut().zip(TAIL_POINTS) {
                *out = width * point;
            }
            let _ = tails.push(AttachedChain {
                chain: Chain::new(x, y, gap * 0.5 * rng.range(0.7, 0.9), 120.0, &sizes),
                radian: rng.range(0.0, PI / 5.0) * if index % 2 == 0 { 1.0 } else { -1.0 },
                position,
            });
        }

        let mut back_fins = Vec::new();
        let mut sizes = [0.0; BACK_FIN_POINTS.len()];
        for (out, point) in sizes.iter_mut().zip(BACK_FIN_POINTS) {
            *out = 0.0 * point;
        }
        let _ = back_fins.push(AttachedChain {
            chain: Chain::new(x, y, gap * 1.5, 120.0, &sizes),
            radian: 0.0,
            position: 3,
        });

        Self {
            gap,
            body,
            fins,
            tails,
            back_fins,
            bounds: Bounds {
                left: x,
                right: x,
                top: y,
                bottom: y,
            },
            cube: Cube::new(x, y, width * 0.15, scene.viewport_width, rng),
            ripple_cooldown_ms: rng.range(400.0, 800.0) as u32,
            last_ripple_time_ms: now_ms,
            show_rig: false,
            on_leave: false,
            end_leave: false,
            leave_target: Point {
                x: -100.0 * scene.spatial_scale,
                y: -100.0 * scene.spatial_scale,
            },
        }
    }

    pub fn update(&mut self, width: f32, height: f32, rng: &mut Random) {
        if self.on_leave {
            self.cube.chase(self.leave_target.x, self.leave_target.y);
            if self.bounds.right < 0.0
                || self.bounds.left > width
                || self.bounds.bottom < 0.0
                || self.bounds.top > height
            {
                self.end_leave = true;
            }
        }
        self.cube.update(width, height, !self.on_leave, rng);
    }

    pub fn move_body(&mut self, width: f32, height: f32) {
        let position = self.cube.position();
        self.body.free_move(position.x, position.y, width, height);
        for attached in &mut self.back_fins {
            let start = self.body.circles[attached.position].position();
            let next = self.body.circles[attached.position + 1].position();
            attached.chain.constrain_move(
                start.x,
                start.y,
                find_tangent(start, next) + attached.radian,
                0.0,
            );
        }
        for (index, attached) in self.fins.iter_mut().enumerate() {
            let start = self.body.circles[attached.position].position();
            let next = self.body.circles[attached.position + 1].position();
            attached.chain.constrain_move(
                start.x,
                start.y,
                find_tangent(start, next) + attached.radian,
                if index <= 1 { 0.3 } else { 0.8 },
            );
        }
        for attached in &mut self.tails {
            let start = self.body.circles[attached.position].position();
            let next = self.body.circles[attached.position + 1].position();
            attached.chain.constrain_move(
                start.x,
                start.y,
                find_tangent(start, next) + attached.radian,
                0.3,
            );
        }
        self.update_bounds();
    }

    pub fn draw_body<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565, stroke_width: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        self.body.draw_outline(target, fill, stroke, stroke_width);
    }

    pub fn draw_fins<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565, stroke_width: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        for fin in &self.fins {
            fin.chain.draw_outline(target, fill, stroke, stroke_width);
        }
    }

    pub fn draw_tail<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565, stroke_width: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        for tail in &self.tails {
            tail.chain.draw_outline(target, fill, stroke, stroke_width);
        }
    }

    pub fn draw_back_fin<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565, stroke_width: u32)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        for back_fin in &self.back_fins {
            let end_position = back_fin.position + BACK_FIN_POINTS.len() + 1;
            let fin_point = back_fin.chain.circles[BACK_FIN_POINTS.len() - 1].position();
            let start_point = self.body.circles[back_fin.position + 1].position();
            let end_point = self.body.circles[end_position].position();
            let mut path: Vec<Point, 32> = Vec::new();
            let _ = path.push(start_point);
            append_quadratic(&mut path, start_point, fin_point, end_point);

            let mut current = end_point;
            for index in (back_fin.position + 2..=end_position).rev() {
                let control = self.body.circles[index].position();
                let previous = self.body.circles[index - 1].position();
                let endpoint = Point {
                    x: (control.x + previous.x) / 2.0,
                    y: (control.y + previous.y) / 2.0,
                };
                append_quadratic(&mut path, current, control, endpoint);
                current = endpoint;
            }
            append_quadratic(
                &mut path,
                current,
                self.body.circles[back_fin.position + 2].position(),
                start_point,
            );
            draw_polygon(target, &path, fill, stroke, stroke_width);
        }
    }

    pub fn draw_eyes<D>(&self, target: &mut D, fill: Rgb565, stroke: Rgb565)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let first = self.body.circles[0].position();
        let second = self.body.circles[1].position();
        let radian = find_tangent(first, second);
        for eye_radian in [radian + PI / 4.0, radian - PI / 4.0] {
            let distance = self.body.circles[2].d * 0.5;
            let x = first.x + distance * libm::cosf(eye_radian);
            let y = first.y + distance * libm::sinf(eye_radian);
            draw_filled_circle(target, x, y, self.gap * 0.4, fill, stroke);
        }
    }

    pub fn draw_rig<D>(&self, target: &mut D, color: Rgb565)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        self.body.draw_rig(target, color);
        for fin in &self.fins {
            fin.chain.draw_rig(target, color);
        }

        for tail in &self.tails {
            tail.chain.draw_rig(target, color);
        }
    }

    fn update_bounds(&mut self) {
        let mut bounds = Bounds {
            left: f32::MAX,
            right: f32::MIN,
            top: f32::MAX,
            bottom: f32::MIN,
        };
        for circle in &self.body.circles {
            bounds.left = bounds.left.min(circle.x);
            bounds.right = bounds.right.max(circle.x);
            bounds.top = bounds.top.min(circle.y);
            bounds.bottom = bounds.bottom.max(circle.y);
        }
        bounds.left -= self.gap;
        bounds.right += self.gap;
        bounds.top -= self.gap;
        bounds.bottom += self.gap;
        self.bounds = bounds;
    }

    pub fn trigger_dash(&mut self) {
        self.cube.dash(libm::atan2f(
            self.cube.vy * self.cube.direction_y,
            self.cube.vx * self.cube.direction_x,
        ));
    }

    pub fn trigger_directional_dash(&mut self, x: f32, y: f32) {
        self.cube
            .dash(find_tangent(self.bounds.center(), Point { x, y }) + PI);
    }

    pub fn is_dashing(&self) -> bool {
        self.velocity() > self.cube.v_max
    }
    pub fn velocity(&self) -> f32 {
        sqrt(self.cube.vx * self.cube.vx + self.cube.vy * self.cube.vy)
    }
    pub fn width(&self) -> f32 {
        self.body.circles[4].d
    }

    pub fn create_ripple(&mut self, now_ms: u64, spatial_scale: f32, rng: &mut Random) -> Ripple {
        let center = self.body.circles[self.body.circles.len() / 2];
        let intensity = map(
            self.velocity(),
            self.cube.v_max,
            self.cube.v_dash,
            0.0,
            100.0,
        );
        self.ripple_cooldown_ms = rng.range(400.0, 800.0) as u32;
        Ripple::new(center.x, center.y, intensity, 0.0, now_ms, spatial_scale)
    }
}

fn append_quadratic(points: &mut Vec<Point, 32>, start: Point, control: Point, end: Point) {
    for step in 1..=4 {
        let t = step as f32 / 4.0;
        let inverse = 1.0 - t;
        let _ = points.push(Point {
            x: inverse * inverse * start.x + 2.0 * inverse * t * control.x + t * t * end.x,
            y: inverse * inverse * start.y + 2.0 * inverse * t * control.y + t * t * end.y,
        });
    }
}
