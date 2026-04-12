use crate::models::effect::{StoreHandlerErr, StoreHandlerTrait};
use crate::models::enums::Tabs;
use crate::{get_ui_state, set_ui_state};
use slint::{ComponentHandle as _, Weak};
use tmd_ui::{GlobalStore, TmdApp};

#[derive(Default)]
pub struct SelectedTabHandler;

impl StoreHandlerTrait<Tabs> for SelectedTabHandler {
    fn on_set(window_weak: &Weak<TmdApp>, value: Tabs) -> Result<(), StoreHandlerErr> {
        set_ui_state(window_weak, move |tmd_app| {
            let global_store = tmd_app.global::<GlobalStore>();

            global_store.set_activeTab(value.into());
        })
        .map_err(|_| StoreHandlerErr::FailToSet)?;

        Ok(())
    }

    fn on_get(window_weak: &Weak<TmdApp>) -> Result<Tabs, StoreHandlerErr> {
        let result = get_ui_state(window_weak, |app| {
            let global_store = app.global::<GlobalStore>();
            global_store.get_activeTab()
        })
        .map_err(|_| StoreHandlerErr::FailToGet)?;

        let tab = match result {
            tmd_ui::Tabs::Home => Tabs::Home,
            tmd_ui::Tabs::Ble => Tabs::Bluetooth,
            tmd_ui::Tabs::Unit => Tabs::Unit,
        };

        Ok(tab)
    }
}
