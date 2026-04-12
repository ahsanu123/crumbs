use crate::{
    models::{effect::Effect, key_event::KeyEvent},
    stores::HandleOnKeyEventTrait,
};

mod is_ble_on_handler;

use defmt::info;
use is_ble_on_handler::IsBleOnHandler;
use slint::Weak;
use tmd_ui::TmdApp;

#[derive(Default)]
pub struct BleOptionStore {
    is_ble_on: Effect<bool, IsBleOnHandler>,

    app_weak: Weak<TmdApp>,
}

impl BleOptionStore {
    pub fn new(app_weak: Weak<TmdApp>) -> Self {
        Self {
            is_ble_on: Default::default(),
            app_weak,
        }
    }

    pub fn handle_turn_on_ble(&mut self) {
        info!("handle_turn_on_ble");
    }

    pub fn handle_turn_off_ble(&mut self) {
        info!("handle_turn_off_ble");
    }

    pub fn set_ble_is_on(&mut self, is_ble_on: bool) {
        self.is_ble_on
            .set(&self.app_weak, is_ble_on)
            .expect("BleOptionStore, fail to set ble_is_on");
    }
}

impl HandleOnKeyEventTrait for BleOptionStore {
    fn on_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::Up => info!("BleOptionStore, got up event, nothing todo"),

            KeyEvent::Down => info!("BleOptionStore, got up event, nothing todo"),

            KeyEvent::Right => self.handle_turn_on_ble(),

            KeyEvent::Left => self.handle_turn_off_ble(),
        }
    }
}
