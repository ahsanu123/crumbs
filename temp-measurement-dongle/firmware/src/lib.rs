#![no_std]

use crate::{
    models::key_event::KeyEvent,
    stores::{BleOptionStore, GlobalStore, HomeStore, SettingStore},
};
use embassy_sync::once_lock::OnceLock;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use slint::{EventLoopError, Weak};
use static_cell::StaticCell;
use tmd_ui::TmdApp;

extern crate alloc;

pub mod display_models;
pub mod initializers;
pub mod platforms;
pub mod stores;
pub mod tasks;

pub(crate) mod models;

pub static TMD_APP_WEAK: OnceLock<Weak<TmdApp>> = OnceLock::new();

pub static BLE_OPTION_STORE: OnceLock<Mutex<CriticalSectionRawMutex, BleOptionStore>> =
    OnceLock::new();

pub static GLOBAL_STORE: OnceLock<Mutex<CriticalSectionRawMutex, GlobalStore>> = OnceLock::new();

pub static HOME_STORE: OnceLock<Mutex<CriticalSectionRawMutex, HomeStore>> = OnceLock::new();

pub static SETTING_STORE: OnceLock<Mutex<CriticalSectionRawMutex, SettingStore>> = OnceLock::new();

pub static INTERFACE_BUFFER: StaticCell<[u8; 312]> = StaticCell::new();

pub static DISPLAY_HEIGHT: usize = 172;
pub static DISPLAY_WIDTH: usize = 320;

const INPUT_PUBSUB_BUFFER_CAPACITY: usize = 4;
const INPUT_PUBSUB_SUBSCRIBER_CAPACITY: usize = 2;
const INPUT_PUBSUB_PUBLISHER_CAPACITY: usize = 1;

pub static INPUT_PUBSUB: PubSubChannel<
    CriticalSectionRawMutex,
    KeyEvent,
    INPUT_PUBSUB_BUFFER_CAPACITY,
    INPUT_PUBSUB_SUBSCRIBER_CAPACITY,
    INPUT_PUBSUB_PUBLISHER_CAPACITY,
> = PubSubChannel::new();

pub fn set_ui_state<F>(app_weak: &Weak<TmdApp>, setter_fn: F) -> Result<(), EventLoopError>
where
    F: FnOnce(TmdApp) + Send + 'static,
{
    let weak = app_weak.clone();
    slint::invoke_from_event_loop(move || {
        if let Some(main_window) = weak.upgrade() {
            setter_fn(main_window)
        }
    })
}
