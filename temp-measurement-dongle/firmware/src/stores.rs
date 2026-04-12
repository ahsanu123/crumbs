mod ble_option_store;
mod global_store;
mod home_store;
mod setting_store;

pub use ble_option_store::*;
pub use global_store::*;
pub use home_store::*;
pub use setting_store::*;

use crate::models::key_event::KeyEvent;

pub trait HandleOnKeyEventTrait {
    fn on_key_event(&mut self, key: KeyEvent);
}
