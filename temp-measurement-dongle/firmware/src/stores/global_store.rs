use crate::{
    GLOBAL_STORE,
    models::{effect::Effect, enums::Tabs},
};
mod selected_tab_handler;

use selected_tab_handler::SelectedTabHandler;

#[derive(Default)]
pub struct GlobalStore {
    pub selected_tab: Effect<Tabs, SelectedTabHandler>,
}

pub async fn handle_on_global_key_up() {
    let mut global_store = GLOBAL_STORE.get().await.lock().await;

    let selected_tab = global_store
        .selected_tab
        .get_internal_val()
        .await
        .expect("fail to get selected tab internal val");

    match selected_tab {
        Tabs::Home => {
            global_store
                .selected_tab
                .set(Tabs::Unit)
                .await
                .expect("global_store, fail to get selected_tab");
        }
        Tabs::Bluetooth => {
            global_store
                .selected_tab
                .set(Tabs::Home)
                .await
                .expect("global_store, fail to get selected_tab");
        }
        Tabs::Unit => {
            global_store
                .selected_tab
                .set(Tabs::Bluetooth)
                .await
                .expect("global_store, fail to get selected_tab");
        }
    }
}

pub async fn handle_on_global_key_down() {
    let mut global_store = GLOBAL_STORE.get().await.lock().await;

    let selected_tab = global_store
        .selected_tab
        .get_internal_val()
        .await
        .expect("fail to get selected tab internal val");

    match selected_tab {
        Tabs::Home => {
            global_store
                .selected_tab
                .set(Tabs::Bluetooth)
                .await
                .expect("global_store, fail to get selected_tab");
        }
        Tabs::Bluetooth => {
            global_store
                .selected_tab
                .set(Tabs::Unit)
                .await
                .expect("global_store, fail to get selected_tab");
        }
        Tabs::Unit => {
            global_store
                .selected_tab
                .set(Tabs::Home)
                .await
                .expect("global_store, fail to get selected_tab");
        }
    }
}
