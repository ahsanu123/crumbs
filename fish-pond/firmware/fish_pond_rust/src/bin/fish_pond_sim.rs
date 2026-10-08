use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window, sdl2::Keycode,
};
use fish_pond_rust::{ChaseDirection, Pond, SceneConfig};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 172;

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(WIDTH, HEIGHT));
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Fish Pond", &output_settings);
    let mut config = SceneConfig::new(WIDTH, HEIGHT);
    config.seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let mut pond = Pond::new(config);
    let mut previous = Instant::now();

    'running: loop {
        let now = Instant::now();
        let elapsed_ms = now
            .duration_since(previous)
            .as_millis()
            .min(u32::MAX as u128) as u32;
        previous = now;
        pond.update(elapsed_ms);
        pond.render(&mut display);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    pond.add_ripple(point.x as f32, point.y as f32, 100.0);
                    pond.dash_at(point.x as f32, point.y as f32);
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Escape => break 'running,
                    Keycode::A => pond.chase_direction = Some(ChaseDirection::BottomLeft),
                    Keycode::D => pond.chase_direction = Some(ChaseDirection::BottomRight),
                    Keycode::W => pond.chase_direction = Some(ChaseDirection::TopCenter),
                    Keycode::R => pond.set_show_rig(!pond.show_rig),
                    Keycode::V => pond.remove_fish(0),
                    _ => {}
                },
                SimulatorEvent::KeyUp {
                    keycode: Keycode::A | Keycode::D | Keycode::W,
                    ..
                } => {
                    pond.chase_direction = None;
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}
