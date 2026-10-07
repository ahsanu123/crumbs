//! # Example: Pacman
//!
//! An example displaying an animated Pacman.

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Radiant(f64);

pub trait GetVertices {
    fn vertices(&self) -> Vec<Point>;
}

pub trait GetCenter {
    fn get_center(&self) -> Point;
}

impl GetVertices for Triangle {
    fn vertices(&self) -> Vec<Point> {
        self.vertices.into()
    }
}

impl GetCenter for Triangle {
    fn get_center(&self) -> Point {
        Point::new(
            (self.vertices[0].x + self.vertices[1].x + self.vertices[2].x) / 3,
            (self.vertices[0].y + self.vertices[1].y + self.vertices[2].y) / 3,
        )
    }
}

fn rotate<IP>(triangle: IP, degre: Radiant) -> Triangle
where
    IP: GetVertices + Transform + GetCenter,
{
    let center = triangle.get_center();
    let centered_triangle = triangle.translate(center * -1);

    let rotated_point: Vec<Point> = centered_triangle
        .vertices()
        .into_iter()
        .map(|mut p| {
            let cos_res = libm::cos(degre.0);
            let sin_res = libm::sin(degre.0);

            let x = p.x as f64;
            let y = p.y as f64;

            p.x = (x * cos_res - y * sin_res) as i32;
            p.y = (x * sin_res + y * cos_res) as i32;

            p
        })
        .collect();

    let new_triangle = Triangle::from_slice(&rotated_point);
    new_triangle.translate(center)
}

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(160, 120));

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("triangle", &output_settings);

    let start = Instant::now();

    'running: loop {
        display.clear(Rgb565::WHITE)?;
        let elapsed = start.elapsed().as_secs_f64();

        let rotation_speed = core::f64::consts::PI / 2.0;
        let angle = elapsed * rotation_speed;

        let triangle = Triangle::new(Point::new(32, 16), Point::new(16, 48), Point::new(48, 48));

        rotate(triangle, Radiant(angle))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::CSS_HOT_PINK, 1))
            .draw(&mut display)?;

        triangle
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::MAGENTA, 1))
            .draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
}
