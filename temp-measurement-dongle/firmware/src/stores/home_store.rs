use crate::{
    models::{effect::Effect, key_event::KeyEvent},
    stores::{HandleOnKeyEventTrait, handle_on_global_key_down, handle_on_global_key_up},
};
mod is_charging_handler;
mod temperature_handler;

use defmt::info;
use is_charging_handler::IsChargingHandler;
use temperature_handler::TemperatureHandler;

#[derive(Default)]
pub struct HomeStore {
    pub temperature: Effect<f32, TemperatureHandler>,
    pub is_charging: Effect<bool, IsChargingHandler>,
}

impl HandleOnKeyEventTrait for HomeStore {
    async fn on_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::Up => handle_on_global_key_up().await,

            KeyEvent::Down => handle_on_global_key_down().await,

            KeyEvent::Right => info!("HomeStore, got right event, nothing todo"),

            KeyEvent::Left => info!("HomeStore, got left event, nothing todo"),
        }
    }
}
