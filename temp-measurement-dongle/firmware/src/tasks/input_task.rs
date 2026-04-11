use crate::{INPUT_PUBSUB, models::key_event::KeyEvent};
use embassy_futures::select::{Either4, select4};
use esp_hal::gpio::Input;

const INPUT_TASK_POOL_SIZE: usize = 2;

#[embassy_executor::task(pool_size = INPUT_TASK_POOL_SIZE)]
pub async fn input_task(
    mut in1: Input<'static>,
    mut in2: Input<'static>,
    mut in3: Input<'static>,
    mut in4: Input<'static>,
) {
    let publisher = INPUT_PUBSUB.publisher().expect("fail to create publisher");

    loop {
        let event = select4(
            in1.wait_for_falling_edge(),
            in2.wait_for_falling_edge(),
            in3.wait_for_falling_edge(),
            in4.wait_for_falling_edge(),
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
