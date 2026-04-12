use crate::{INPUT_PUBSUB, models::key_event::KeyEvent};
use embassy_futures::select::{Either4, select4};
use esp_hal::gpio::Input;

#[embassy_executor::task]
pub async fn input_task(
    mut key_up: Input<'static>,
    mut key_right: Input<'static>,
    mut key_down: Input<'static>,
    mut key_left: Input<'static>,
) {
    let publisher = INPUT_PUBSUB.publisher().expect("fail to create publisher");

    loop {
        let event = select4(
            key_up.wait_for_falling_edge(),
            key_right.wait_for_falling_edge(),
            key_down.wait_for_falling_edge(),
            key_left.wait_for_falling_edge(),
        )
        .await;

        match event {
            Either4::First(_) => publisher.publish(KeyEvent::Up).await,
            Either4::Second(_) => publisher.publish(KeyEvent::Right).await,
            Either4::Third(_) => publisher.publish(KeyEvent::Down).await,
            Either4::Fourth(_) => publisher.publish(KeyEvent::Left).await,
        };
    }
}
