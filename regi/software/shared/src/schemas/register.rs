use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum BitType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Bit {
    pub bit_id: i32,
    pub bit_ordinal: u32,
    pub bit_type: BitType,
    pub reset_val: u32,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Register {
    pub register_id: i32,
    pub name: String,
    pub description: String,
    pub address: u32,
    pub bits: Vec<Bit>,
}
