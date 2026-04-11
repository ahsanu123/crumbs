use crate::{
    models::{effect::Effect, enums::Units, key_event::KeyEvent},
    stores::{HandleOnKeyEventTrait, handle_on_global_key_down, handle_on_global_key_up},
};

mod unit_temperature_handler;

use unit_temperature_handler::UnitTemperatureHandler;

#[derive(Default)]
pub struct SettingStore {
    pub temperature_unit: Effect<Units, UnitTemperatureHandler>,
}

impl HandleOnKeyEventTrait for SettingStore {
    async fn on_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::Up => handle_on_global_key_up().await,

            KeyEvent::Down => handle_on_global_key_down().await,

            KeyEvent::Right => {
                let selected_unit = self
                    .temperature_unit
                    .get_internal_val()
                    .await
                    .expect("setting_store fail to get internal val");

                match selected_unit {
                    Units::Celcius => self.temperature_unit.set(Units::Reamur).await,

                    Units::Reamur => self.temperature_unit.set(Units::Fahrenheit).await,

                    Units::Fahrenheit => self.temperature_unit.set(Units::Celcius).await,
                };
            }

            KeyEvent::Left => {
                let selected_unit = self
                    .temperature_unit
                    .get_internal_val()
                    .await
                    .expect("setting_store fail to get internal val");

                match selected_unit {
                    Units::Celcius => self.temperature_unit.set(Units::Fahrenheit).await,

                    Units::Reamur => self.temperature_unit.set(Units::Celcius).await,

                    Units::Fahrenheit => self.temperature_unit.set(Units::Reamur).await,
                };
            }
        }
    }
}
