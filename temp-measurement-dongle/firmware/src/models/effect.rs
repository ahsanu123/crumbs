use crate::{TMD_APP_WEAK, models::get_id_trait::GetIdTrait};
use alloc::vec::Vec;
use core::marker::PhantomData;
use tmd_ui::TmdApp;

#[derive(Debug)]
pub enum StoreHandlerErr {
    FailToSet,
    FailToGet,
}

pub trait StoreHandlerTrait<T> {
    fn on_set(
        window_weak: &slint::Weak<TmdApp>,
        value: T,
    ) -> impl Future<Output = Result<(), StoreHandlerErr>>;

    fn on_get(
        window_weak: &slint::Weak<TmdApp>,
    ) -> impl Future<Output = Result<T, StoreHandlerErr>>;
}

#[derive(Default)]
pub struct Effect<T, H>
where
    T: Clone,
    H: StoreHandlerTrait<T>,
{
    value: T,

    _handler: PhantomData<H>,
}

impl<T, H> Effect<T, H>
where
    T: Clone,
    H: StoreHandlerTrait<T>,
{
    pub async fn set(&mut self, value: T) -> Result<(), StoreHandlerErr> {
        self.value = value.clone();

        let window_weak = TMD_APP_WEAK.get().await;
        H::on_set(window_weak, value).await?;

        Ok(())
    }

    pub async fn get(&mut self) -> Result<T, StoreHandlerErr> {
        let window_weak = TMD_APP_WEAK.get().await;
        let value = H::on_get(window_weak).await?;

        Ok(value)
    }

    pub async fn get_internal_val(&self) -> Result<T, StoreHandlerErr> {
        Ok(self.value.clone())
    }
}

impl<T, H> Effect<Vec<T>, H>
where
    T: Clone + GetIdTrait,
    H: StoreHandlerTrait<Vec<T>>,
{
    pub async fn insert(&mut self, value: T) -> Result<(), StoreHandlerErr> {
        let target_id = value.get_id();

        if let Some(existing_value) = self.value.iter_mut().find(|v| v.get_id() == target_id) {
            *existing_value = value;
        } else {
            self.value.push(value);
        }

        let window_weak = TMD_APP_WEAK.get().await;
        H::on_set(window_weak, self.value.clone()).await?;

        Ok(())
    }
}
