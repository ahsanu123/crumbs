use crate::{
    get_ui_state,
    models::effect::{StoreHandlerErr, StoreHandlerTrait},
    set_ui_state,
};
use slint::{ComponentHandle, Weak};
use tmd_ui::{BleOptionSlintStore, TmdApp};

#[derive(Default)]
pub struct IsBleOnHandler;

impl StoreHandlerTrait<bool> for IsBleOnHandler {
    fn on_set(window_weak: &Weak<TmdApp>, value: bool) -> Result<(), StoreHandlerErr> {
        set_ui_state(window_weak, move |tmd_app| {
            let bleoption_store = tmd_app.global::<BleOptionSlintStore>();

            bleoption_store.set_isBleOn(value);
        })
        .map_err(|_| StoreHandlerErr::FailToSet)?;

        Ok(())
    }

    fn on_get(window_weak: &Weak<TmdApp>) -> Result<bool, StoreHandlerErr> {
        get_ui_state(window_weak, |app| {
            let bleslint_store = app.global::<BleOptionSlintStore>();
            bleslint_store.get_isBleOn()
        })
        .map_err(|_| StoreHandlerErr::FailToGet)
    }
}
