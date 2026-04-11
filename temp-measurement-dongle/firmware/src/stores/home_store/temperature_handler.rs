use crate::models::effect::StoreHandlerTrait;

#[derive(Default)]
pub struct TemperatureHandler;

impl StoreHandlerTrait<f32> for TemperatureHandler {
    async fn on_set(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
        value: f32,
    ) -> Result<(), crate::models::effect::StoreHandlerErr> {
        todo!()
    }

    async fn on_get(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
    ) -> Result<f32, crate::models::effect::StoreHandlerErr> {
        todo!()
    }
}
