use crate::initializator::{MutexedVolumeManagerType, VolumeManagerType};

use embassy_time::{Duration, Timer};
use embedded_sdmmc::VolumeIdx;
use typed_builder::TypedBuilder;

pub struct DummyTimeSource;

impl embedded_sdmmc::TimeSource for DummyTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 56,
            zero_indexed_month: 8,
            zero_indexed_day: 17,
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
        self.with_cs(|volman| {
            let volume0 = volman
                .open_volume(VolumeIdx(0))
                .expect("fail to read volume 0");

            defmt::info!("Volume 0: {:?}", volume0);

            let root_dir = volume0.open_root_dir().expect("fail to open root dir");

            let dummy_txt = root_dir
                .open_file_in_dir(
                    "dummy_sd_test.txt",
                    embedded_sdmmc::Mode::ReadWriteCreateOrAppend,
                )
                .expect("fail to open file");

            defmt::info!("success to open create or open dummy file");

            dummy_txt
                .write(b"dummy message inserted\n")
                .expect("fail to write ");

            defmt::info!("success write dummy message to file");
        });

        Timer::after(Duration::from_secs(3)).await;
    }
}

#[embassy_executor::task]
pub async fn run_volume_manager_task(volman_task: VolumeManagerTask<'static>) {
    // NOTE: do some subscription here
    loop {
        volman_task.run().await;
    }
}
