use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Interpreter {
    pub interpreter_id: i32,
    pub name: String,
    pub description: String,
    pub register_ids: Vec<i32>,
    pub translator_id: i32,
}
