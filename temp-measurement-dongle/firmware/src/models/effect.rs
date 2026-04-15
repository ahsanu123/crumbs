use crate::models::get_id_trait::GetIdTrait;
use alloc::vec::Vec;
use core::marker::PhantomData;
use slint::Weak;
use tmd_ui::TmdApp;

#[derive(Debug)]
pub enum StoreHandlerErr {
    FailToSet,
    FailToGet,
}

pub trait StoreHandlerTrait<T> {
    fn on_set(window_weak: Weak<TmdApp>, value: T) -> Result<(), StoreHandlerErr>;

    fn on_get(window_weak: Weak<TmdApp>) -> Result<T, StoreHandlerErr>;
}

#[derive(Default)]
pub struct Effect<T, H>
where
    T: Clone,
    H: StoreHandlerTrait<T>,
{
    value: T,
    app_weak: Weak<TmdApp>,

    _handler: PhantomData<H>,
}

impl<T, H> Effect<T, H>
where
    T: Clone + Default,
    H: StoreHandlerTrait<T>,
{
    pub fn new(app_weak: Weak<TmdApp>) -> Self {
        Self {
            app_weak,
            value: Default::default(),

            _handler: PhantomData,
        }
    }

    pub fn set(&mut self, value: T) -> Result<(), StoreHandlerErr> {
        self.value = value.clone();
        H::on_set(self.app_weak.clone(), value)?;

        Ok(())
    }

    pub fn get(&mut self, window_weak: &Weak<TmdApp>) -> Result<T, StoreHandlerErr> {
        let value = H::on_get(window_weak.clone())?;
        Ok(value)
    }

    pub fn get_internal_val(&self) -> Result<T, StoreHandlerErr> {
        Ok(self.value.clone())
    }
}

impl<T, H> Effect<Vec<T>, H>
where
    T: Clone + GetIdTrait,
    H: StoreHandlerTrait<Vec<T>>,
{
    pub fn insert(&mut self, value: T) -> Result<(), StoreHandlerErr> {
        let target_id = value.get_id();

        if let Some(existing_value) = self.value.iter_mut().find(|v| v.get_id() == target_id) {
            *existing_value = value;
        } else {
            self.value.push(value);
        }

        H::on_set(self.app_weak.clone(), self.value.clone())?;

        Ok(())
    }
}
