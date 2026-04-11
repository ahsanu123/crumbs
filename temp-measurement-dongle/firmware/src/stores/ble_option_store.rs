use crate::{
    models::{effect::Effect, key_event::KeyEvent},
    stores::{HandleOnKeyEventTrait, handle_on_global_key_down, handle_on_global_key_up},
};

mod is_ble_on_handler;

use defmt::info;
use is_ble_on_handler::IsBleOnHandler;

#[derive(Default)]
pub struct BleOptionStore {
    pub is_ble_on: Effect<bool, IsBleOnHandler>,
}

impl BleOptionStore {
    pub async fn handle_turn_on_ble(&mut self) {
        info!("handle_turn_on_ble");
        todo!()
    }

    pub async fn handle_turn_off_ble(&mut self) {
        info!("handle_turn_off_ble");
        todo!()
    }
}

impl HandleOnKeyEventTrait for BleOptionStore {
    async fn on_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::Up => handle_on_global_key_up().await,

            KeyEvent::Down => handle_on_global_key_down().await,

            KeyEvent::Right => self.handle_turn_on_ble().await,

            KeyEvent::Left => self.handle_turn_off_ble().await,
        }
    }
}
