use embassy_sync::mutex::Mutex;

use crate::{
    BLE_OPTION_STORE, GLOBAL_STORE, HOME_STORE, SETTING_STORE,
    stores::{BleOptionStore, GlobalStore, HomeStore, SettingStore},
};

pub fn init_stores() {
    BLE_OPTION_STORE
        .init(Mutex::new(BleOptionStore::default()))
        .map_err(|_| "")
        .expect("fail to init BleOptionStore");

    GLOBAL_STORE
        .init(Mutex::new(GlobalStore::default()))
        .map_err(|_| "")
        .expect("fail to init BleOptionStore");

    HOME_STORE
        .init(Mutex::new(HomeStore::default()))
        .map_err(|_| "")
        .expect("fail to init BleOptionStore");

    SETTING_STORE
        .init(Mutex::new(SettingStore::default()))
        .map_err(|_| "")
        .expect("fail to init BleOptionStore");
}
