use crate::{
    models::{effect::Effect, enums::Units, key_event::KeyEvent},
    stores::HandleOnKeyEventTrait,
};

mod unit_temperature_handler;

use defmt::info;
use slint::Weak;
use tmd_ui::TmdApp;
use unit_temperature_handler::UnitTemperatureHandler;

#[derive(Default)]
pub struct SettingStore {
    temperature_unit: Effect<Units, UnitTemperatureHandler>,
}

impl SettingStore {
    pub fn new(app_weak: Weak<TmdApp>) -> Self {
        Self {
            temperature_unit: Effect::new(app_weak),
        }
    }
}

impl HandleOnKeyEventTrait for SettingStore {
    fn on_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::Up => info!("SettingStore, got up key already handle by global_store"),

            KeyEvent::Down => info!("SettingStore, got down key already handle by global_store"),

            KeyEvent::Left => {
                let selected_unit = self
                    .temperature_unit
                    .get_internal_val()
                    .expect("setting_store fail to get internal val");

                match selected_unit {
                    Units::Celcius => self
                        .temperature_unit
                        .set(Units::Reamur)
                        .expect("setting_store, fail to get unit"),

                    Units::Reamur => self
                        .temperature_unit
                        .set(Units::Fahrenheit)
                        .expect("setting_store, fail to get unit"),

                    Units::Fahrenheit => self
                        .temperature_unit
                        .set(Units::Celcius)
                        .expect("setting_store, fail to get unit"),
                };
            }

            KeyEvent::Right => {
                let selected_unit = self
                    .temperature_unit
                    .get_internal_val()
                    .expect("setting_store fail to get internal val");

                match selected_unit {
                    Units::Celcius => self
                        .temperature_unit
                        .set(Units::Fahrenheit)
                        .expect("setting_store, fail to get unit"),

                    Units::Reamur => self
                        .temperature_unit
                        .set(Units::Celcius)
                        .expect("setting_store, fail to get unit"),

                    Units::Fahrenheit => self
                        .temperature_unit
                        .set(Units::Reamur)
                        .expect("setting_store, fail to get unit"),
                };
            }
        }
    }
}
