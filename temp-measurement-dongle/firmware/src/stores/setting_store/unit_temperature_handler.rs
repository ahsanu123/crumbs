use crate::models::effect::{StoreHandlerErr, StoreHandlerTrait};
use crate::models::enums::Units;
use crate::{get_ui_state, set_ui_state};
use slint::{ComponentHandle as _, Weak};
use tmd_ui::{SettingStoreSlint, TmdApp};

#[derive(Default)]
pub struct UnitTemperatureHandler;

impl StoreHandlerTrait<Units> for UnitTemperatureHandler {
    fn on_set(window_weak: &Weak<TmdApp>, value: Units) -> Result<(), StoreHandlerErr> {
        set_ui_state(window_weak, move |tmd_app| {
            let setting_store = tmd_app.global::<SettingStoreSlint>();
            setting_store.set_selectedUnit(value.into());
        })
        .map_err(|_| StoreHandlerErr::FailToSet)?;

        Ok(())
    }

    fn on_get(window_weak: &Weak<TmdApp>) -> Result<Units, StoreHandlerErr> {
        let result = get_ui_state(window_weak, |tmd_app| {
            let setting_store = tmd_app.global::<SettingStoreSlint>();
            setting_store.get_selectedUnit()
        })
        .map_err(|_| StoreHandlerErr::FailToGet)?;

        let unit = match result {
            tmd_ui::Units::Celcius => Units::Celcius,
            tmd_ui::Units::Reamur => Units::Reamur,
            tmd_ui::Units::Fahrenheit => Units::Fahrenheit,
        };

        Ok(unit)
    }
}
