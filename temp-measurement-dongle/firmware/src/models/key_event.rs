#[derive(Clone)]
pub enum MediatorEvent {
    Key(KeyEvent),
    IsCharging(bool),
    Max31865(f32),   // temperature
    BleOption(bool), // On or Off
}

#[derive(Clone)]
pub enum KeyEvent {
    Up,
    Right,
    Down,
    Left,
}
