use crate::initializator::{MutexedVolumeManagerType, VolumeManagerType};

use typed_builder::TypedBuilder;

pub struct DummyTimeSource;

impl embedded_sdmmc::TimeSource for DummyTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[derive(TypedBuilder)]
pub struct VolumeManagerTask<'m> {
    volman_instance: &'m MutexedVolumeManagerType,
}

impl<'m> VolumeManagerTask<'m> {
    fn with_cs<F>(&self, func: F)
    where
        F: Fn(&VolumeManagerType),
    {
        critical_section::with(|cs| {
            let sdmut = self.volman_instance.borrow_ref_mut(cs);
            func(&sdmut);
        });
    }
}

pub trait TaskTrait<'l> {
    fn run(&'l self) -> impl Future<Output = ()>;
}

impl<'m> TaskTrait<'m> for VolumeManagerTask<'m> {
    async fn run(&self) {
        loop {
            self.with_cs(|volman| {
                // let cardType = sdcard.get_card_type();
            });
            todo!()
        }
    }
}

#[embassy_executor::task]
pub async fn run_volume_manager_task(volman_task: VolumeManagerTask<'static>) {
    // NOTE: do some subscription here

    volman_task.run().await;
}
