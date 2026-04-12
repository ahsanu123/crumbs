use crate::{BLE_OPERATION_PUBSUB, models::ble_operation_event::BleOperationEvent};
use defmt::info;
use embassy_sync::pubsub::WaitResult;

#[embassy_executor::task]
pub async fn ble_operation_task() {
    let mut ble_op_sub = BLE_OPERATION_PUBSUB
        .subscriber()
        .expect("BLE_OPERATION_PUBSUB, fail to create subscription");

    loop {
        let message = ble_op_sub.next_message().await;

        match message {
            WaitResult::Lagged(amount) => info!("lag {} message", amount),

            WaitResult::Message(operation_msg) => match operation_msg {
                BleOperationEvent::TurnOn => info!("TurnOn, turn on ble"),
                BleOperationEvent::TurnOff => info!("TurnOff, turn off ble"),
                BleOperationEvent::Restart => info!("Restart, maybe un-needed, but restart ble"),
            },
        }
    }
}
