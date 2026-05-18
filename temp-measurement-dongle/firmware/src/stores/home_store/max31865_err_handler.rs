use crate::{
    get_ui_state,
    models::effect::{StoreHandlerErr, StoreHandlerTrait},
    set_ui_state,
};
use max31865::error::Max31865Err;
use slint::{ComponentHandle as _, Weak};
use tmd_ui::{HomeSlintStore, Max31865SlintErr, OptionMax31865Err, TmdApp};

fn convert_to_optional_max31865_slint_err(maybe_err: Option<Max31865Err>) -> OptionMax31865Err {
    if let Some(err) = maybe_err {
        let max31865_err: Max31865SlintErr = match err {
            Max31865Err::Spi(spi_err) => match spi_err {
                max31865::error::SpiErr::FailToRead => Max31865SlintErr::SPIFailToRead,
                max31865::error::SpiErr::FailToWrite => Max31865SlintErr::SPIFailToWrite,
            },
            Max31865Err::Hardware(hardware_err) => match hardware_err {
                max31865::error::HardwareErr::OverOrUnderVoltage => {
                    Max31865SlintErr::HardwareOverOrUnderVoltage
                }
                max31865::error::HardwareErr::RtdinForceMinusOpen => {
                    Max31865SlintErr::HardwareRtdinForceMinusOpen
                }
                max31865::error::HardwareErr::RefinForceMinusOpen => {
                    Max31865SlintErr::HardwareRefinForceMinusOpen
                }
                max31865::error::HardwareErr::RefinGreatherThanPoint85TimeVBias => {
                    Max31865SlintErr::HardwareRefinGreatherThanPoint85TimeVBias
                }
                max31865::error::HardwareErr::RtdLowThreshold => {
                    Max31865SlintErr::HardwareRtdLowThreshold
                }
                max31865::error::HardwareErr::RtdHighThreshold => {
                    Max31865SlintErr::HardwareRtdHighThreshold
                }
            },
            Max31865Err::Operation(_) => Max31865SlintErr::HardwareRtdHighThreshold,
        };

        OptionMax31865Err {
            hasValue: true,
            value: max31865_err,
        }
    } else {
        OptionMax31865Err {
            hasValue: false,
            value: Max31865SlintErr::HardwareRtdHighThreshold,
        }
    }
}

fn convert_to_optional_max31865_err(maybe_err: OptionMax31865Err) -> Option<Max31865Err> {
    if maybe_err.hasValue {
        match maybe_err.value {
            Max31865SlintErr::SPIFailToRead => {
                Some(Max31865Err::Spi(max31865::error::SpiErr::FailToRead))
            }
            Max31865SlintErr::SPIFailToWrite => {
                Some(Max31865Err::Spi(max31865::error::SpiErr::FailToWrite))
            }

            Max31865SlintErr::HardwareOverOrUnderVoltage => Some(Max31865Err::Hardware(
                max31865::error::HardwareErr::OverOrUnderVoltage,
            )),
            Max31865SlintErr::HardwareRtdinForceMinusOpen => Some(Max31865Err::Hardware(
                max31865::error::HardwareErr::RtdinForceMinusOpen,
            )),
            Max31865SlintErr::HardwareRefinForceMinusOpen => Some(Max31865Err::Hardware(
                max31865::error::HardwareErr::RefinForceMinusOpen,
            )),
            Max31865SlintErr::HardwareRefinGreatherThanPoint85TimeVBias => {
                Some(Max31865Err::Hardware(
                    max31865::error::HardwareErr::RefinGreatherThanPoint85TimeVBias,
                ))
            }
            Max31865SlintErr::HardwareRtdLowThreshold => Some(Max31865Err::Hardware(
                max31865::error::HardwareErr::RtdLowThreshold,
            )),
            Max31865SlintErr::HardwareRtdHighThreshold => Some(Max31865Err::Hardware(
                max31865::error::HardwareErr::RtdHighThreshold,
            )),
        }
    } else {
        None
    }
}

#[derive(Default)]
pub struct Max31865ErrHandler;

impl StoreHandlerTrait<Option<Max31865Err>> for Max31865ErrHandler {
    fn on_set(
        window_weak: Weak<TmdApp>,
        value: Option<Max31865Err>,
    ) -> Result<(), StoreHandlerErr> {
        set_ui_state(window_weak, move |tmd_app| {
            let home_store = tmd_app.global::<HomeSlintStore>();

            let max31865_err_value = convert_to_optional_max31865_slint_err(value);
            home_store.set_max31865Err(max31865_err_value);
        })
        .map_err(|_| StoreHandlerErr::FailToSet)?;

        Ok(())
    }

    fn on_get(window_weak: Weak<TmdApp>) -> Result<Option<Max31865Err>, StoreHandlerErr> {
        let slint_err = get_ui_state(window_weak, |tmd_app| {
            let home_store = tmd_app.global::<HomeSlintStore>();
            home_store.get_max31865Err()
        })
        .map_err(|_| StoreHandlerErr::FailToGet)?;
        Ok(convert_to_optional_max31865_err(slint_err))
    }
}
