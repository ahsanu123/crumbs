use alloc::string::String;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pubsub::PubSubChannel;

const VOLMAN_BUF_CAP: usize = 4;
const VOLMAN_SUB_CAP: usize = 2;
const VOLMAN_PUB_CAP: usize = 2;

#[derive(Clone)]
pub enum VolumManagerEvent {
    ReadConfig(String),
    ConfigFileChanged,
}

pub static MEDIATOR_PUBSUB: PubSubChannel<
    CriticalSectionRawMutex,
    VolumManagerEvent,
    VOLMAN_BUF_CAP,
    VOLMAN_SUB_CAP,
    VOLMAN_PUB_CAP,
> = PubSubChannel::new();
