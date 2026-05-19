use flatbuffers::FlatBufferBuilder;
use regi_repl::repl_sch_generated::regi_repl::{
    I2cWrite16Args, I2cWrite16ArgsArgs, Message, MessageArgs, Operations,
};
use rustyline::completion::FilenameCompleter;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent};
use rustyline::{Completer, Helper, Hinter, Validator};

use clap::{Parser, Subcommand};

use std::borrow::Cow::{self, Owned};

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Highlighter for MyHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

fn parse_u8(s: &str) -> Result<u8, String> {
    if let Some(hex) = s.strip_prefix("0x") {
        u8::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else if let Some(bin) = s.strip_prefix("0b") {
        u8::from_str_radix(bin, 2).map_err(|e| e.to_string())
    } else {
        s.parse::<u8>().map_err(|e| e.to_string())
    }
}

#[derive(Parser, Debug)]
#[command(name = "cli")]
struct Cli {
    #[command(subcommand)]
    protocols: Protocols,
}

#[derive(Subcommand, Debug)]
enum Protocols {
    Spi {
        #[command(subcommand)]
        operations: SpiOperations,
    },

    I2c {
        #[command(subcommand)]
        operations: I2cOperations,
    },

    Exit,
}

#[derive(Subcommand, Debug)]
enum SpiOperations {
    Read8 {
        #[arg(value_parser = parse_u8)]
        address: u8,
    },
    Read16 {
        #[arg(value_parser = parse_u8)]
        address: u8,
    },

    Write8 {
        #[arg(value_parser = parse_u8)]
        address: u8,
        #[arg(value_parser = parse_u8)]
        value1: u8,
    },
    Write16 {
        #[arg(value_parser = parse_u8)]
        address: u8,
        #[arg(value_parser = parse_u8)]
        value1: u8,
        #[arg(value_parser = parse_u8)]
        value2: u8,
    },
}

#[derive(Subcommand, Debug)]
enum I2cOperations {
    Read8 {
        #[arg(value_parser = parse_u8)]
        address: u8,
    },
    Read16 {
        #[arg(value_parser = parse_u8)]
        address: u8,
    },

    Write8 {
        #[arg(value_parser = parse_u8)]
        address: u8,
        #[arg(value_parser = parse_u8)]
        value1: u8,
    },
    Write16 {
        #[arg(value_parser = parse_u8)]
        address: u8,
        #[arg(value_parser = parse_u8)]
        value1: u8,
        #[arg(value_parser = parse_u8)]
        value2: u8,
    },
}

// To debug rustyline:
// RUST_LOG=rustyline=debug cargo run --example example 2> debug.log
// or for Windows Powershell:
// $env:RUST_LOG="rustyline=debug"; cargo run --example example 2> debug.log
fn main() -> rustyline::Result<()> {
    // env_logger::init();
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .build();

    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter::new(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config)?;

    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut builder = FlatBufferBuilder::new();

    loop {
        let line = rl.readline(">> ");

        match line {
            Ok(input) => {
                rl.add_history_entry(input.as_str())?;

                let args = std::iter::once("cli")
                    .chain(input.split_whitespace())
                    .collect::<Vec<_>>();

                let cli = match Cli::try_parse_from(args) {
                    Ok(cli) => cli,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };

                match cli.protocols {
                    Protocols::Exit => {
                        break;
                    }
                    Protocols::Spi { operations } => match operations {
                        SpiOperations::Read8 { address } => {
                            println!("spi read8 at address: {:#X}", address)
                        }
                        SpiOperations::Read16 { address } => {
                            println!("spi read16 at address: {:#X}", address)
                        }
                        SpiOperations::Write8 { address, value1 } => {
                            println!(
                                "spi write8 at address: {:#X}, value: {:#X}",
                                address, value1
                            )
                        }
                        SpiOperations::Write16 {
                            address,
                            value1,
                            value2,
                        } => println!(
                            "spi write16 at address: {:#X}, value1: {:#X}, value2: {:#X}",
                            address, value1, value2
                        ),
                    },
                    Protocols::I2c { operations } => match operations {
                        I2cOperations::Read8 { address } => {
                            println!("i2c read8 at address: {:#X}", address)
                        }
                        I2cOperations::Read16 { address } => {
                            println!("i2c read16 at address: {:#X}", address)
                        }
                        I2cOperations::Write8 { address, value1 } => {
                            println!(
                                "i2c write8 at address: {:#X}, value: {:#X}",
                                address, value1
                            )
                        }
                        I2cOperations::Write16 {
                            address,
                            value1,
                            value2,
                        } => {
                            let i2c_write16_arg = I2cWrite16Args::create(
                                &mut builder,
                                &I2cWrite16ArgsArgs {
                                    address,
                                    value1,
                                    value2,
                                },
                            );

                            let message = Message::create(
                                &mut builder,
                                &MessageArgs {
                                    operations_type: Operations::I2cWrite16Args,
                                    operations: Some(i2c_write16_arg.as_union_value()),
                                },
                            );

                            builder.finish(message, None);

                            let data = builder.finished_data();

                            for bit in data {
                                print!("{:02X} ", bit);
                            }

                            println!("=====================================================");
                            println!(
                                "i2c write16 at address: {:#X}, value1: {:#X}, value2: {:#X}",
                                address, value1, value2
                            );
                        }
                    },
                }
            }

            Err(_) => {
                break;
            }
        }
    }
    rl.append_history("history.txt")
}

#[cfg(test)]
mod rustyline_ex_test {
    use regi_repl::repl_sch_generated::regi_repl::root_as_message;

    use super::*;

    #[test]
    fn parse_as_message() {
        let buf: Vec<u8> = vec![
            0x0C, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0E, 0x00, 0x07, 0x00, 0x08, 0x00, 0x08, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00,
            0x08, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x0A,
            0x01, 0x21,
        ];

        let message = root_as_message(&buf).expect("fail to parse");

        match message.operations_type() {
            Operations::I2cWrite16Args => {
                let op = message.operations_as_i2c_write_16_args().unwrap();

                println!("address = {:#X}", op.address());
                println!("value1  = {:#X}", op.value1());
                println!("value2  = {:#X}", op.value2());
            }

            _ => {
                println!("unknown operation");
            }
        }
    }
}
