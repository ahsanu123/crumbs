use core::f32::consts::PI;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    geometry::Point as EgPoint,
    pixelcolor::Rgb565,
    prelude::{Primitive, RgbColor},
    primitives::{Circle, Line, PrimitiveStyle},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Random {
    rng: fastrand::Rng,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: fastrand::Rng::with_seed(seed),
        }
    }

    pub fn unit(&mut self) -> f32 {
        self.rng.f32()
    }

    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        self.rng.f32() * (max - min) + min
    }

    pub fn sign(&mut self) -> f32 {
        let is_positive = self.rng.bool();
        // Block LLVM from tracking the boolean's value in a no_std project
        let is_positive = core::hint::black_box(is_positive);

        if is_positive { 1.0 } else { -1.0 }
    }
}

pub fn find_angle_between(point_a: Point, point_b: Point, point_c: Point) -> f32 {
    let vector_1 = [point_b.x - point_a.x, point_b.y - point_a.y];
    let vector_2 = [point_c.x - point_a.x, point_c.y - point_a.y];
    let product = vector_1[0] * vector_2[0] + vector_1[1] * vector_2[1];
    let lengths = sqrt(vector_1[0] * vector_1[0] + vector_1[1] * vector_1[1])
        * sqrt(vector_2[0] * vector_2[0] + vector_2[1] * vector_2[1]);
    if lengths == 0.0 {
        0.0
    } else {
        libm::acosf((product / lengths).clamp(-1.0, 1.0))
    }
}

pub fn find_tangent(point_a: Point, point_b: Point) -> f32 {
    libm::atan2f(point_b.y - point_a.y, point_b.x - point_a.x)
}

pub fn is_on_left(point_a: Point, point_b: Point, point_c: Point) -> bool {
    (point_b.x - point_a.x) * (point_c.y - point_a.y)
        - (point_b.y - point_a.y) * (point_c.x - point_a.x)
        > 0.0
}

pub fn find_position(point: Point, radian: f32, length: f32) -> Point {
    Point {
        x: point.x + length * libm::cosf(radian),
        y: point.y + length * libm::sinf(radian),
    }
}

pub fn lerp(begin: f32, target: f32, increase: f32) -> f32 {
    begin + (target - begin) * increase
}

// This intentionally preserves the TypeScript implementation, including its
// missing `min2` offset.
pub fn map(value: f32, min1: f32, max1: f32, min2: f32, max2: f32) -> f32 {
    let percentage = (value - min1) / (max1 - min1);
    let _ = min2;
    (max2 - min2) * percentage
}

pub fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))
}

pub fn normalize_vector(vector: Point, magnitude: f32) -> Point {
    find_position(
        Point::default(),
        find_tangent(Point::default(), vector),
        magnitude,
    )
}

pub fn random_point_outside_rect(width: f32, height: f32, rng: &mut Random) -> Point {
    let x = rng.range(-width, width * 2.0);
    let y = if x < -width * 0.5 || x > width * 1.5 {
        rng.range(-height, height * 2.0)
    } else if rng.unit() < 0.5 {
        rng.range(-height, -height * 0.5)
    } else {
        rng.range(height * 1.5, height * 2.0)
    };
    Point { x, y }
}

pub fn sqrt(value: f32) -> f32 {
    libm::sqrtf(value)
}

pub const fn rgb565(red: u8, green: u8, blue: u8) -> Rgb565 {
    Rgb565::new(
        ((red as u16 * 31 + 127) / 255) as u8,
        ((green as u16 * 63 + 127) / 255) as u8,
        ((blue as u16 * 31 + 127) / 255) as u8,
    )
}

pub fn draw_circle<D>(target: &mut D, x: f32, y: f32, diameter: f32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let diameter = diameter.max(1.0) as u32;
    let top_left = EgPoint::new(
        (x - diameter as f32 / 2.0) as i32,
        (y - diameter as f32 / 2.0) as i32,
    );
    let _ = target.draw_iter(
        Circle::new(top_left, diameter)
            .into_styled(PrimitiveStyle::with_stroke(color, 1))
            .pixels(),
    );
}

pub fn draw_filled_circle<D>(
    target: &mut D,
    x: f32,
    y: f32,
    diameter: f32,
    fill: Rgb565,
    stroke: Rgb565,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let diameter = diameter.max(1.0) as u32;
    let top_left = EgPoint::new(
        (x - diameter as f32 / 2.0) as i32,
        (y - diameter as f32 / 2.0) as i32,
    );
    let style = PrimitiveStyle::with_fill(fill);
    let _ = Circle::new(top_left, diameter)
        .into_styled(style)
        .draw(target);
    let _ = Circle::new(top_left, diameter)
        .into_styled(PrimitiveStyle::with_stroke(stroke, 1))
        .draw(target);
}

pub fn line<D>(target: &mut D, a: Point, b: Point, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let _ = Line::new(to_eg(a), to_eg(b))
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(target);
}

pub fn draw_polygon<D, const N: usize>(
    target: &mut D,
    points: &heapless::Vec<Point, N>,
    fill: Rgb565,
    stroke: Rgb565,
    stroke_width: u32,
) where
    D: DrawTarget<Color = Rgb565>,
{
    if points.len() < 3 {
        return;
    }
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for point in points {
        min_y = min_y.min(point.y as i32);
        max_y = max_y.max(point.y as i32);
    }
    for y in min_y..=max_y {
        let scan_y = y as f32 + 0.5;
        let mut intersections: heapless::Vec<f32, N> = heapless::Vec::new();
        for index in 0..points.len() {
            let a = points[index];
            let b = points[(index + 1) % points.len()];
            if (a.y <= scan_y && b.y > scan_y) || (b.y <= scan_y && a.y > scan_y) {
                let x = a.x + (scan_y - a.y) * (b.x - a.x) / (b.y - a.y);
                let _ = intersections.push(x);
            }
        }
        intersections
            .as_mut_slice()
            .sort_unstable_by(f32::total_cmp);
        for index in (0..intersections.len()).step_by(2) {
            if index + 1 >= intersections.len() {
                break;
            }
            let _ = Line::new(
                EgPoint::new(intersections[index] as i32, y),
                EgPoint::new(intersections[index + 1] as i32, y),
            )
            .into_styled(PrimitiveStyle::with_stroke(fill, 1))
            .draw(target);
        }
    }
    for index in 0..points.len() {
        let _ = Line::new(
            to_eg(points[index]),
            to_eg(points[(index + 1) % points.len()]),
        )
        .into_styled(PrimitiveStyle::with_stroke(stroke, stroke_width))
        .draw(target);
    }
}

pub fn to_eg(point: Point) -> EgPoint {
    EgPoint::new(point.x as i32, point.y as i32)
}

pub const WHITE: Rgb565 = Rgb565::WHITE;
pub const PI2: f32 = 2.0 * PI;
