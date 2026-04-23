use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum TranslatorType {
    Pair,
    Pair2,
    Pair3,
    Pair4,
    Pair5,
    Pair6,
    Pair7,
    Pair8,
    Pair9,
    Pair10,
    Pair11,
    Pair12,
    Pair13,
    Pair14,
    Pair15,
    Pair16,

    Formula,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BitPairData {
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Formula {
    Str(String),
    KeyPair(HashMap<i32, BitPairData>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Translator {
    pub translator_id: i32,
    pub translator_type: TranslatorType,
    pub formula: Formula,
}
