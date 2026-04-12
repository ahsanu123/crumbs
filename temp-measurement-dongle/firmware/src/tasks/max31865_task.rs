use crate::{MEDIATOR_PUBSUB, initializers::Max31865Type, models::key_event::MediatorEvent};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn max31865_task(mut max31865_dev: Max31865Type) {
    let mediator_pub = MEDIATOR_PUBSUB
        .publisher()
        .expect("fail to create publisher");

    loop {
        let temperature = max31865_dev.temperature();

        mediator_pub
            .publish(MediatorEvent::Max31865(temperature))
            .await;

        Timer::after_secs(1).await;
    }
}
