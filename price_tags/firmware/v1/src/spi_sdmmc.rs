use aligned::A4;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx, SdCard};
use exfat_slim::asynchronous::{
    BlockDevice as ExfatBlockDevice,
    fs::{self, BlockDeviceError},
};
use mbr_nostd::{MasterBootRecord, PartitionTable};

const BLOCK_SIZE: usize = 512;

#[derive(Debug)]
pub enum AgnosticError {
    Io(embedded_sdmmc::sdcard::spi::Error),
    Exfat(fs::Error),
}

impl BlockDeviceError for AgnosticError {
    fn to_error_code(&self) -> u8 {
        match self {
            AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::BadState) => 1,
            AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::CrcError(_, _)) => 2,
            AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::CardNotFound) => 3,
            AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::TimeoutReadBuffer) => 5,
            AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::TimeoutWaitNotBusy) => 6,
            // TODO:
            // AgnosticError::Io(sdmmc::Error::SignalingSwitchFailed) => 4,
            // AgnosticError::Io(sdmmc::Error::UnsupportedCardType) => 7,
            // AgnosticError::Io(sdmmc::Error::UnsupportedCardVersion) => 8,
            _ => 0,
        }
    }

    fn no_card(&self) -> bool {
        matches!(
            self,
            AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::TimeoutReadBuffer)
                | AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::TimeoutWaitNotBusy)
                | AgnosticError::Io(embedded_sdmmc::sdcard::spi::Error::CardNotFound)
        )
    }
}

impl From<fs::Error> for AgnosticError {
    fn from(value: fs::Error) -> Self {
        Self::Exfat(value)
    }
}

impl From<embedded_sdmmc::sdcard::spi::Error> for AgnosticError {
    fn from(value: embedded_sdmmc::sdcard::spi::Error) -> Self {
        Self::Io(value)
    }
}

pub trait AgnosticExfatDev {
    fn init(&mut self) -> impl Future<Output = Result<(), AgnosticError>>;
    fn read_mbr(&mut self) -> impl Future<Output = Result<u32, AgnosticError>>;
}

pub struct AgnosticSdmmc<S, D>
where
    S: embedded_hal::spi::SpiDevice,
    D: embedded_hal::delay::DelayNs,
{
    dev: SdCard<S, D>,
    pub offset: u32,
    is_init: bool,
}

impl<S, D> AgnosticSdmmc<S, D>
where
    S: embedded_hal::spi::SpiDevice,
    D: embedded_hal::delay::DelayNs,
{
    pub fn new(dev: SdCard<S, D>) -> Self {
        Self {
            dev,
            offset: 0,
            is_init: false,
        }
    }
}

impl<S, D> AgnosticExfatDev for AgnosticSdmmc<S, D>
where
    S: embedded_hal::spi::SpiDevice,
    D: embedded_hal::delay::DelayNs,
{
    async fn init(&mut self) -> Result<(), AgnosticError> {
        if !self.is_init {
            self.dev.num_blocks()?;
            self.offset = self.read_mbr().await?;
            self.is_init = true;
        }

        Ok(())
    }

    async fn read_mbr(&mut self) -> Result<u32, AgnosticError> {
        let mut blocks_data = [Block::new()];

        self.dev.read(&mut blocks_data, BlockIdx(0))?;

        let mbr = MasterBootRecord::from_bytes(&*blocks_data[0]).unwrap();
        let entries = mbr.partition_table_entries();
        let _num_entries = entries.len();
        let entry = entries.first().unwrap();
        let offset = entry.logical_block_address;

        // info!(
        //     "reading master boot record - offset: {} partition_type: {} sector_count: {}, num_entrties: {}",
        //     offset,
        //     entry.partition_type.to_mbr_tag_byte(),
        //     entry.sector_count,
        //     num_entries
        // );

        Ok(offset)
    }
}

impl<S, D> ExfatBlockDevice<BLOCK_SIZE> for AgnosticSdmmc<S, D>
where
    S: embedded_hal::spi::SpiDevice,
    D: embedded_hal::delay::DelayNs,
{
    type Error = AgnosticError;
    type Align = A4;

    async fn read(
        &mut self,
        block_address: u32,
        data: &mut [aligned::Aligned<Self::Align, [u8; BLOCK_SIZE]>],
    ) -> Result<(), Self::Error> {
        self.init().await?;
        let block_idx = self.offset + block_address;

        for (i, output) in data.iter_mut().enumerate() {
            let idx = block_idx + i as u32;
            let mut blocks = [Block::new()];

            self.dev
                .read(&mut blocks, BlockIdx(idx))
                .map_err(AgnosticError::from)?;

            output.copy_from_slice(blocks[0].as_slice());
        }

        Ok(())
    }

    async fn write(
        &mut self,
        block_address: u32,
        data: &[aligned::Aligned<Self::Align, [u8; BLOCK_SIZE]>],
    ) -> Result<(), Self::Error> {
        self.init().await?;
        let block_idx = self.offset + block_address;
        assert_ne!(block_idx, 0); // you really don't want to write to this block on your sd card

        for (i, input) in data.iter().enumerate() {
            let idx = block_idx + i as u32;

            // Prevent overwriting the MBR at physical block zero.
            assert_ne!(idx, 0);

            let mut blocks = [Block::new()];

            blocks[0].as_mut_slice().copy_from_slice(&input[..]);

            self.dev
                .write(&blocks, BlockIdx(idx))
                .map_err(AgnosticError::from)?;
        }

        Ok(())
    }

    async fn size(&mut self) -> Result<u64, Self::Error> {
        let numblock = self.dev.num_blocks()?;

        Ok(numblock.0 as u64)
    }
}

pub async fn file_system_task_from_spi<SPI, D>(spi: SPI, delay: D)
where
    SPI: embedded_hal::spi::SpiDevice,
    D: embedded_hal::delay::DelayNs,
{
    let sdcard = SdCard::new(spi, delay);
    let agnostic_sdmmc = AgnosticSdmmc::new(sdcard);

    fs::fs_actor_task::<_, _, 4>(agnostic_sdmmc, None).await;
}

pub async fn file_system_task_from_sdcard<SPI, D>(sdcard: SdCard<SPI, D>)
where
    SPI: embedded_hal::spi::SpiDevice,
    D: embedded_hal::delay::DelayNs,
{
    let agnostic_sdmmc = AgnosticSdmmc::new(sdcard);
    fs::fs_actor_task::<_, _, 4>(agnostic_sdmmc, None).await;
}
