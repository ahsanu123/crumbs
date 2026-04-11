use crate::initializers::DrawBufferType;
use crate::models::enums::Tabs;
use crate::platforms::esp32s3_async::backend::Esp32AsyncBackend;
use crate::stores::HandleOnKeyEventTrait as _;
use crate::{
    BLE_OPTION_STORE, DISPLAY_HEIGHT, DISPLAY_WIDTH, GLOBAL_STORE, HOME_STORE, INPUT_PUBSUB,
    SETTING_STORE, TMD_APP_WEAK,
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

    TMD_APP_WEAK
        .init(weak_tmd_app)
        .map_err(|_| "")
        .expect("fail to init TMD_APP_WEAK");

    let mut sub = INPUT_PUBSUB
        .subscriber()
        .expect("fail to make subscribtion");

    loop {
        slint::platform::update_timers_and_animations();

        // NOTE: Loop Idea
        // Wait For PB input,
        // then mutate store, store will change slint store,
        // then finally update window based on new slint store value,
        // so without new input event, no new ui update.

        let wait_result = sub.next_message().await;

        let global_store = GLOBAL_STORE.get().await.lock().await;

        let selected_tab = global_store
            .selected_tab
            .get_internal_val()
            .await
            .expect("fail to get GLOBAL_STORE internal value");

        match wait_result {
            WaitResult::Lagged(amount) => info!("lag {} message", amount),

            WaitResult::Message(key) => match selected_tab {
                Tabs::Home => {
                    let mut home_store = HOME_STORE.get().await.lock().await;
                    home_store.on_key_event(key);
                }
                Tabs::Bluetooth => {
                    let mut ble_option_store = BLE_OPTION_STORE.get().await.lock().await;
                    ble_option_store.on_key_event(key);
                }
                Tabs::Unit => {
                    let mut setting_store = SETTING_STORE.get().await.lock().await;
                    setting_store.on_key_event(key);
                }
            },
        };

        window.draw_if_needed(|renderer| {
            info!("render by line!");
            renderer.render_by_line(&mut draw_buffer);
        });

        Timer::after_millis(50).await;
    }
}
