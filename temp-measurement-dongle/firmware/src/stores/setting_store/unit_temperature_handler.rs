use crate::models::{effect::StoreHandlerTrait, enums::Units};

#[derive(Default)]
pub struct UnitTemperatureHandler;

impl StoreHandlerTrait<Units> for UnitTemperatureHandler {
    async fn on_set(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
        value: Units,
    ) -> Result<(), crate::models::effect::StoreHandlerErr> {
        todo!()
    }

    async fn on_get(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
    ) -> Result<Units, crate::models::effect::StoreHandlerErr> {
        todo!()
    }
}
