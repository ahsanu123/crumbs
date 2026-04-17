use crate::{
    models::{effect::Effect, enums::Units, key_event::KeyEvent},
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
}

impl HomeStore {
    pub fn new(app_weak: Weak<TmdApp>) -> Self {
        Self {
            temperature: Effect::new(app_weak.clone()),
            is_charging: Effect::new(app_weak.clone()),
        }
    }

    pub fn set_temperature(&mut self, temp: f32, unit: Units) {
        let temp = match unit {
            Units::Celcius => temp,
            Units::Reamur => temp * (4.0 / 5.0),
            Units::Fahrenheit => (temp * (9.0 / 5.0)) + 32.0,
        };

        self.temperature
            .set(temp)
            .expect("HomeStore, fail to set temperature");
    }

    pub fn set_is_charging(&mut self, is_charging: bool) {
        self.is_charging
            .set(is_charging)
            .expect("HomeStore, fail to set is_charging");
    }
}

impl HandleOnKeyEventTrait for HomeStore {
    fn on_key_event(&mut self, _key: KeyEvent) {
        info!("HomeStore, got key event, nothing todo");
    }
}
