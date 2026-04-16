#![no_std]

use crate::{
    models::{ble_operation_event::BleOperationEvent, key_event::MediatorEvent},
    stores::{BleOptionStore, GlobalStore, HomeStore, SettingStore},
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pubsub::PubSubChannel;
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

pub static BLE_OPTION_STORE: StaticCell<BleOptionStore> = StaticCell::new();

pub static GLOBAL_STORE: StaticCell<GlobalStore> = StaticCell::new();

pub static HOME_STORE: StaticCell<HomeStore> = StaticCell::new();

pub static SETTING_STORE: StaticCell<SettingStore> = StaticCell::new();

pub static LCD_DEV_INTERFACE_BUFFER: StaticCell<[u8; 312]> = StaticCell::new();

pub static DISPLAY_HEIGHT: usize = 128;
pub static DISPLAY_WIDTH: usize = 160;

const MEDIATOR_PUBSUB_BUFFER_CAPACITY: usize = 4;
const MEDIATOR_PUBSUB_SUBSCRIBER_CAPACITY: usize = 2;
const MEDIATOR_PUBSUB_PUBLISHER_CAPACITY: usize = 2;

pub static MEDIATOR_PUBSUB: PubSubChannel<
    CriticalSectionRawMutex,
    MediatorEvent,
    MEDIATOR_PUBSUB_BUFFER_CAPACITY,
    MEDIATOR_PUBSUB_SUBSCRIBER_CAPACITY,
    MEDIATOR_PUBSUB_PUBLISHER_CAPACITY,
> = PubSubChannel::new();

const BLE_OP_PUBSUB_BUFFER_CAPACITY: usize = 4;
const BLE_OP_PUBSUB_SUBSCRIBER_CAPACITY: usize = 1;
const BLE_OP_PUBSUB_PUBLISHER_CAPACITY: usize = 1;

pub static BLE_OPERATION_PUBSUB: PubSubChannel<
    CriticalSectionRawMutex,
    BleOperationEvent,
    BLE_OP_PUBSUB_BUFFER_CAPACITY,
    BLE_OP_PUBSUB_SUBSCRIBER_CAPACITY,
    BLE_OP_PUBSUB_PUBLISHER_CAPACITY,
> = PubSubChannel::new();

pub fn set_ui_state<F>(app_weak: Weak<TmdApp>, setter_fn: F) -> Result<(), EventLoopError>
where
    F: FnOnce(TmdApp) + Send + 'static,
{
    if let Some(application) = app_weak.upgrade() {
        setter_fn(application);
        Ok(())
    } else {
        Err(EventLoopError::NoEventLoopProvider)
    }
}

pub fn get_ui_state<F, T>(app_weak: Weak<TmdApp>, getter_fn: F) -> Result<T, EventLoopError>
where
    F: FnOnce(TmdApp) -> T + Send + 'static,
{
    let weak = app_weak.clone();

    if let Some(app) = weak.upgrade() {
        let result = getter_fn(app);

        return Ok(result);
    }

    Err(EventLoopError::NoEventLoopProvider)
}
