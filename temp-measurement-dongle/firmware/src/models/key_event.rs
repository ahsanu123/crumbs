use max31865::error::Max31865Err;

#[derive(Clone)]
pub enum MediatorEvent {
    Key(KeyEvent),
    IsCharging(bool),
    Max31865(f32),            // temperature
    Max31865Err(Max31865Err), // temperature
    BleOption(bool),          // On or Off
}

#[derive(Clone)]
pub enum KeyEvent {
    Up,
    Right,
    Down,
    Left,
}
