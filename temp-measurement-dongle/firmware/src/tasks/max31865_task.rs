use crate::{HOME_STORE, MAX31865_SIGNAL, initializers::Max31865Type};
use embassy_time::Timer;

pub enum Max31865Signal {
    DataUpdated,
}

#[embassy_executor::task]
pub async fn max31865_task(mut max31865_dev: Max31865Type) {
    loop {
        max31865_dev.read_rtd();
        let temperature = max31865_dev.temperature();

        let mut home_store = HOME_STORE.get().await.lock().await;
        home_store
            .temperature
            .set(temperature)
            .await
            .expect("max31865_task fail to update temperature");

        MAX31865_SIGNAL.signal(Max31865Signal::DataUpdated);
        Timer::after_millis(100).await;
    }
}
