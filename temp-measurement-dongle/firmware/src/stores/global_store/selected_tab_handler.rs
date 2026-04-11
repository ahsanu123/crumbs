use crate::models::{effect::StoreHandlerTrait, enums::Tabs};

#[derive(Default)]
pub struct SelectedTabHandler;

impl StoreHandlerTrait<Tabs> for SelectedTabHandler {
    async fn on_set(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
        value: Tabs,
    ) -> Result<(), crate::models::effect::StoreHandlerErr> {
        todo!()
    }

    async fn on_get(
        window_weak: &slint::Weak<tmd_ui::TmdApp>,
    ) -> Result<Tabs, crate::models::effect::StoreHandlerErr> {
        todo!()
    }
}
