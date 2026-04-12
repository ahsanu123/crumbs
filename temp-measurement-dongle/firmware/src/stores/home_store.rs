use crate::{
    models::{effect::Effect, key_event::KeyEvent},
    stores::HandleOnKeyEventTrait,
};
mod is_charging_handler;
mod temperature_handler;

use defmt::info;
use is_charging_handler::IsChargingHandler;
use slint::Weak;
use temperature_handler::TemperatureHandler;
use tmd_ui::TmdApp;

#[derive(Default)]
pub struct HomeStore {
    temperature: Effect<f32, TemperatureHandler>,
    is_charging: Effect<bool, IsChargingHandler>,

    app_weak: Weak<TmdApp>,
}

impl HomeStore {
    pub fn new(app_weak: Weak<TmdApp>) -> Self {
        Self {
            temperature: Default::default(),
            is_charging: Default::default(),
            app_weak,
        }
    }

    pub fn set_temperature(&mut self, temp: f32) {
        self.temperature
            .set(&self.app_weak, temp)
            .expect("HomeStore, fail to set temperature");
    }

    pub fn set_is_charging(&mut self, is_charging: bool) {
        self.is_charging
            .set(&self.app_weak, is_charging)
            .expect("HomeStore, fail to set is_charging");
    }
}

impl HandleOnKeyEventTrait for HomeStore {
    fn on_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::Up => info!("HomeStore, got up event, nothing todo"),

            KeyEvent::Down => info!("HomeStore, got down event, nothing todo"),

            KeyEvent::Right => info!("HomeStore, got right event, nothing todo"),

            KeyEvent::Left => info!("HomeStore, got left event, nothing todo"),
        }
    }
}
