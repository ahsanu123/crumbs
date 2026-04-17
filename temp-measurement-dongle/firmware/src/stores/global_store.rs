use crate::models::{effect::Effect, enums::Tabs};
mod selected_tab_handler;

use selected_tab_handler::SelectedTabHandler;
use slint::Weak;
use tmd_ui::TmdApp;

#[derive(Default)]
pub struct GlobalStore {
    pub selected_tab: Effect<Tabs, SelectedTabHandler>,
}

impl GlobalStore {
    pub fn new(app_weak: Weak<TmdApp>) -> Self {
        Self {
            selected_tab: Effect::new(app_weak),
        }
    }

    pub fn on_global_key_up(&mut self) {
        let selected_tab = self
            .selected_tab
            .get_internal_val()
            .expect("fail to get selected tab internal val");

        match selected_tab {
            Tabs::Home => {
                self.selected_tab
                    .set(Tabs::Unit)
                    .expect("global_store, fail to get selected_tab");
            }
            Tabs::Bluetooth => {
                self.selected_tab
                    .set(Tabs::Home)
                    .expect("global_store, fail to get selected_tab");
            }
            Tabs::Unit => {
                self.selected_tab
                    .set(Tabs::Bluetooth)
                    .expect("global_store, fail to get selected_tab");
            }
        }
    }
    pub fn on_global_key_down(&mut self) {
        let selected_tab = self
            .selected_tab
            .get_internal_val()
            .expect("fail to get selected tab internal val");

        match selected_tab {
            Tabs::Home => {
                self.selected_tab
                    .set(Tabs::Bluetooth)
                    .expect("global_store, fail to get selected_tab");
            }
            Tabs::Bluetooth => {
                self.selected_tab
                    .set(Tabs::Unit)
                    .expect("global_store, fail to get selected_tab");
            }
            Tabs::Unit => {
                self.selected_tab
                    .set(Tabs::Home)
                    .expect("global_store, fail to get selected_tab");
            }
        }
    }
}
