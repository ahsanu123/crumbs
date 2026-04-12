use crate::{
    get_ui_state,
    models::effect::{StoreHandlerErr, StoreHandlerTrait},
    set_ui_state,
};
use slint::{ComponentHandle as _, Weak};
use tmd_ui::{HomeSlintStore, TmdApp};

#[derive(Default)]
pub struct IsChargingHandler;

impl StoreHandlerTrait<bool> for IsChargingHandler {
    async fn on_set(window_weak: &Weak<TmdApp>, value: bool) -> Result<(), StoreHandlerErr> {
        set_ui_state(window_weak, move |tmd_app| {
            let home_store = tmd_app.global::<HomeSlintStore>();
            home_store.set_isCharging(value);
        })
        .map_err(|_| StoreHandlerErr::FailToSet)?;

        Ok(())
    }

    async fn on_get(window_weak: &Weak<TmdApp>) -> Result<bool, StoreHandlerErr> {
        get_ui_state(window_weak, |tmd_app| {
            let home_store = tmd_app.global::<HomeSlintStore>();
            home_store.get_isCharging()
        })
        .map_err(|_| StoreHandlerErr::FailToGet)
    }
}
