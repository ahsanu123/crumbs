use crate::initializers::DrawBufferType;
use crate::models::enums::Tabs;
use crate::models::key_event::{KeyEvent, MediatorEvent};
use crate::platforms::esp32s3_async::backend::Esp32AsyncBackend;
use crate::stores::{
    BleOptionStore, GlobalStore, HandleOnKeyEventTrait as _, HomeStore, SettingStore,
};
use crate::{
    BLE_OPTION_STORE, DISPLAY_HEIGHT, DISPLAY_WIDTH, GLOBAL_STORE, HOME_STORE, MEDIATOR_PUBSUB,
    SETTING_STORE,
};
use alloc::boxed::Box;
use defmt::info;
use embassy_sync::pubsub::WaitResult;
use embassy_time::Timer;
use esp_hal::gpio::Output;
use slint::ComponentHandle as _;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use tmd_ui::TmdApp;

pub async fn ui_task(mut draw_buffer: DrawBufferType, mut lcd_blk: Output<'static>) {
    let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
    window.set_size(slint::PhysicalSize::new(
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32,
    ));

    let backend = Box::new(Esp32AsyncBackend::new(window.clone()));
    slint::platform::set_platform(backend).expect("fail when set platform");

    info!("slint gui setup complete");

    lcd_blk.set_high();

    let tmd_app = TmdApp::new().expect("cant create Tmd Application");
    tmd_app.show().expect("unable to show tmd_app");

    let weak_tmd_app = tmd_app.as_weak();

    let global_store = GLOBAL_STORE.init(GlobalStore::new(weak_tmd_app.clone()));
    let ble_option_store = BLE_OPTION_STORE.init(BleOptionStore::new(weak_tmd_app.clone()));
    let home_store = HOME_STORE.init(HomeStore::new(weak_tmd_app.clone()));
    let setting_store = SETTING_STORE.init(SettingStore::new(weak_tmd_app.clone()));

    let mut mediator_sub = MEDIATOR_PUBSUB
        .subscriber()
        .expect("fail to make subscribtion");

    loop {
        slint::platform::update_timers_and_animations();

        let selected_tab = global_store
            .selected_tab
            .get_internal_val()
            .expect("global_store, fail to get selected_tab");

        let mediator_msg = mediator_sub.next_message().await;

        match mediator_msg {
            WaitResult::Lagged(amount) => info!("lag {} message", amount),

            WaitResult::Message(message) => match message {
                MediatorEvent::IsCharging(is_charging) => home_store.set_is_charging(is_charging),

                MediatorEvent::Max31865(temperature) => home_store.set_temperature(temperature),

                MediatorEvent::BleOption(is_ble_on) => ble_option_store.set_ble_is_on(is_ble_on),

                MediatorEvent::Key(key_event) => match key_event {
                    KeyEvent::Up => global_store.on_global_key_up(),

                    KeyEvent::Down => global_store.on_global_key_down(),

                    // FIXME: think better approach
                    KeyEvent::Right => match selected_tab {
                        Tabs::Home => home_store.on_key_event(key_event),

                        Tabs::Bluetooth => ble_option_store.on_key_event(key_event),

                        Tabs::Unit => setting_store.on_key_event(key_event),
                    },
                    KeyEvent::Left => match selected_tab {
                        Tabs::Home => home_store.on_key_event(key_event),

                        Tabs::Bluetooth => ble_option_store.on_key_event(key_event),

                        Tabs::Unit => setting_store.on_key_event(key_event),
                    },
                },
            },
        }

        window.draw_if_needed(|renderer| {
            info!("render by line!");
            renderer.render_by_line(&mut draw_buffer);
        });

        Timer::after_millis(50).await;
    }
}
