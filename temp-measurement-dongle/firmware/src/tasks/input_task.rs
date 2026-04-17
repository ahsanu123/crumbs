use crate::{
    MEDIATOR_PUBSUB,
    models::key_event::{KeyEvent, MediatorEvent},
};
use embassy_futures::select::{Either5, select5};
use esp_hal::gpio::Input;

#[embassy_executor::task]
pub async fn input_task(
    mut key_up: Input<'static>,
    mut key_right: Input<'static>,
    mut key_down: Input<'static>,
    mut key_left: Input<'static>,
    mut input_is_charging: Input<'static>,
) {
    let mediator_pub = MEDIATOR_PUBSUB
        .publisher()
        .expect("fail to create publisher");

    loop {
        let event = select5(
            key_up.wait_for_any_edge(),
            key_right.wait_for_any_edge(),
            key_down.wait_for_any_edge(),
            key_left.wait_for_any_edge(),
            input_is_charging.wait_for_any_edge(),
        )
        .await;

        match event {
            Either5::First(_) => {
                if key_up.is_low() {
                    mediator_pub.publish(MediatorEvent::Key(KeyEvent::Up)).await;
                }
            }
            Either5::Second(_) => {
                if key_right.is_low() {
                    mediator_pub
                        .publish(MediatorEvent::Key(KeyEvent::Right))
                        .await
                }
            }
            Either5::Third(_) => {
                if key_down.is_low() {
                    mediator_pub
                        .publish(MediatorEvent::Key(KeyEvent::Down))
                        .await
                }
            }
            Either5::Fourth(_) => {
                if key_left.is_low() {
                    mediator_pub
                        .publish(MediatorEvent::Key(KeyEvent::Left))
                        .await
                }
            }

            Either5::Fifth(_) => {
                let is_charging = input_is_charging.is_low();

                mediator_pub
                    .publish(MediatorEvent::IsCharging(is_charging))
                    .await;
            }
        };
    }
}
