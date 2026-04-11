use crate::models::effect::StoreHandlerTrait;

#[derive(Default)]
pub struct IsBleOnHandler;

impl StoreHandlerTrait<bool> for IsBleOnHandler {
    async fn on_set(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
        value: bool,
    ) -> Result<(), crate::models::effect::StoreHandlerErr> {
        todo!()
    }

    async fn on_get(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
    ) -> Result<bool, crate::models::effect::StoreHandlerErr> {
        todo!()
    }
}
