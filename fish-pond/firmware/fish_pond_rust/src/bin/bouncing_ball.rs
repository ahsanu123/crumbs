#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::Instant;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_println as _;

use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Triangle},
    transform::Transform,
};

use rapier2d::prelude::*;

extern crate alloc;

const WIDTH: usize = 160;
const HEIGHT: usize = 120;
fn normalize_position(pose: &Pose2) -> Pose2 {
    let y = HEIGHT as f32 * 0.85 - pose.translation.y;
    let x = WIDTH as f32 * 0.5 + pose.translation.x;
    Pose2::new(Vector2::new(x, y), 0.0)
}

fn create_screen_boundaries(world: &mut PhysicsWorld) {
    let thickness = 10.0;
    let width = 160.0;
    let height = 120.0;

    // Top wall
    world.insert_collider(
        ColliderBuilder::cuboid(width / 2.0, thickness / 2.0)
            .translation(Vector2::new(0.0, height / 2.0 + thickness / 2.0))
            .restitution(1.0), // Bouncy!
        None,
    );

    // Bottom wall
    world.insert_collider(
        ColliderBuilder::cuboid(width / 2.0, thickness / 2.0)
            .translation(Vector2::new(0.0, -(height / 2.0 + thickness / 2.0)))
            .restitution(1.0),
        None,
    );

    // Left wall
    world.insert_collider(
        ColliderBuilder::cuboid(thickness / 2.0, height / 2.0)
            .translation(Vector2::new(-(width / 2.0 + thickness / 2.0), 0.0))
            .restitution(1.0),
        None,
    );

    // Right wall
    world.insert_collider(
        ColliderBuilder::cuboid(thickness / 2.0, height / 2.0)
            .translation(Vector2::new(width / 2.0 + thickness / 2.0, 0.0))
            .restitution(1.0),
        None,
    );
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32s3 -o alloc -o defmt -o unstable-hal -o esp-backtrace -o embassy -o wifi -o neovim

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_240MHz);

    let peripherals = esp_hal::init(config);

    let (_key_up, _key_left, _key_down, _key_right, _input_is_charging, mut display, mut lcd_blk) =
        fish_pond_rust::initializers::init_devices::init_devices(peripherals);

    let _ = spawner;

    lcd_blk.set_high();

    let mut world = PhysicsWorld::new();
    world.gravity = Vector::new(0.0, -100.0);

    /* Create the ground. */
    world.insert_collider(ColliderBuilder::cuboid(100.0, 0.1).restitution(1.0), None);
    create_screen_boundaries(&mut world);

    let ballrad = 20.0;
    /* Create the bouncing ball. */
    let (ball_body_handle, _) = world.insert(
        RigidBodyBuilder::dynamic().translation(Vector::new(0.0, 30.0)),
        ColliderBuilder::ball(ballrad).restitution(1.0),
    );

    let style = PrimitiveStyle::with_fill(Rgb565::CSS_HOT_PINK);

    let mut kick = Vector2::new(-5000.0, 20.0);
    let mut last_time = 0;

    /* Run the game loop, stepping the simulation once per frame. */
    loop {
        world.step();
        last_time += 1;
        display.clear(Rgb565::WHITE).unwrap();

        let ball_body = world.bodies.get_mut(ball_body_handle).unwrap();

        ball_body.apply_impulse(kick, true);
        if last_time >= 400 {
            kick = -kick;
            last_time = 0;
        }

        let ball_pos = ball_body.position();

        let ball_pos = normalize_position(ball_pos);
        let ball = Circle::new(
            Point {
                x: ball_pos.translation.x as i32,
                y: ball_pos.translation.y as i32,
            },
            ballrad as u32,
        );

        ball.into_styled(style).draw(&mut display);
        display.flush().unwrap();

        Timer::after_millis(1).await;
    }
}
