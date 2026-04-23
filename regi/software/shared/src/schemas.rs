use serde::{Deserialize, Serialize, de};

mod interpreter;
mod register;
mod translator;

pub use interpreter::*;
pub use register::*;
pub use register::*;
pub use translator::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Schema {
    name: String,
    version: i32,
    description: String,

    registers: Vec<Register>,

    interpreters: Vec<Interpreter>,
    translator: Vec<Translator>,
}

#[cfg(test)]
mod schema_test {
    use std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
    };

    use super::*;

    #[test]
    fn test_read_schema_from_file() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/max31865.toml");
        let toml = fs::read_to_string(file_path).expect("fail to open max31865.toml");

        let schema: Schema = toml::de::from_str(&toml).expect("fail to Deserialize");
        println!("{:#?}", schema);
    }

    #[test]
    fn test_write_to_file() {
        let mut pair_map = HashMap::new();
        pair_map.insert(
            0,
            BitPairData {
                description: "Disabled".to_string(),
            },
        );
        pair_map.insert(
            1,
            BitPairData {
                description: "Enabled".to_string(),
            },
        );

        let schema = Schema {
            name: "Motor Controller".to_string(),
            version: 1,
            description: "Schema for controlling industrial motor registers".to_string(),

            registers: vec![
                Register {
                    register_id: 1,
                    name: "STATUS_REG".to_string(),
                    description: "Device status register".to_string(),
                    address: 0x0001,
                    bits: vec![
                        Bit {
                            bit_id: 1,
                            bit_ordinal: 0,
                            bit_type: BitType::ReadOnly,
                            reset_val: 0,
                            description: "Power Ready".to_string(),
                        },
                        Bit {
                            bit_id: 2,
                            bit_ordinal: 1,
                            bit_type: BitType::ReadOnly,
                            reset_val: 0,
                            description: "Fault Active".to_string(),
                        },
                        Bit {
                            bit_id: 3,
                            bit_ordinal: 2,
                            bit_type: BitType::ReadWrite,
                            reset_val: 1,
                            description: "Run Command".to_string(),
                        },
                    ],
                },
                Register {
                    register_id: 2,
                    name: "CONTROL_REG".to_string(),
                    description: "Device control register".to_string(),
                    address: 0x0002,
                    bits: vec![
                        Bit {
                            bit_id: 4,
                            bit_ordinal: 0,
                            bit_type: BitType::WriteOnly,
                            reset_val: 0,
                            description: "Reset Fault".to_string(),
                        },
                        Bit {
                            bit_id: 5,
                            bit_ordinal: 1,
                            bit_type: BitType::ReadWrite,
                            reset_val: 0,
                            description: "Enable Output".to_string(),
                        },
                    ],
                },
            ],

            interpreters: vec![
                Interpreter {
                    interpreter_id: 1,
                    name: "Status Decoder".to_string(),
                    description: "Interpret STATUS_REG values".to_string(),
                    register_ids: vec![1],
                    translator_id: 1,
                },
                Interpreter {
                    interpreter_id: 2,
                    name: "Control Formula".to_string(),
                    description: "Compute control state".to_string(),
                    register_ids: vec![1, 2],
                    translator_id: 2,
                },
            ],

            translator: vec![
                Translator {
                    translator_id: 1,
                    translator_type: TranslatorType::Pair,
                    formula: Formula::KeyPair(pair_map),
                },
                Translator {
                    translator_id: 2,
                    translator_type: TranslatorType::Formula,
                    formula: Formula::Str(
                        "(STATUS_REG & 0x01) == 1 && (CONTROL_REG & 0x02) == 0".to_string(),
                    ),
                },
            ],
        };

        let schema = toml::ser::to_string_pretty(&schema).expect("fail to serialize schema!!");

        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/bq25895.toml");
        fs::write(file_path, schema).expect("fail to write into bq25895");
    }
}
