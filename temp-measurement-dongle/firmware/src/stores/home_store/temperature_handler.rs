use crate::{
    get_ui_state,
    models::effect::{StoreHandlerErr, StoreHandlerTrait},
    set_ui_state,
};
use num_traits::float::FloatCore;
use slint::{ComponentHandle as _, Weak};
use tmd_ui::{HomeSlintStore, TmdApp};

#[derive(Default)]
pub struct TemperatureHandler;

impl StoreHandlerTrait<f32> for TemperatureHandler {
    fn on_set(window_weak: Weak<TmdApp>, value: f32) -> Result<(), StoreHandlerErr> {
        set_ui_state(window_weak, move |tmd_app| {
            let home_store = tmd_app.global::<HomeSlintStore>();

            let value = (value * 1000.0).round() / 1000.0;

            home_store.set_temperature(value);
        })
        .map_err(|_| StoreHandlerErr::FailToSet)?;

        Ok(())
    }

    fn on_get(window_weak: Weak<TmdApp>) -> Result<f32, StoreHandlerErr> {
        get_ui_state(window_weak, |tmd_app| {
            let home_store = tmd_app.global::<HomeSlintStore>();
            home_store.get_temperature()
        })
        .map_err(|_| StoreHandlerErr::FailToGet)
    }
}
