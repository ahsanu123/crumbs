#![no_std]

extern crate alloc;

pub mod communicator;
pub mod initializator;
pub mod parser;
pub mod services;
pub mod tasks;

#[derive(Default)]
pub struct App;
