use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
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

#[derive(Parser, Debug)]
#[command(name = "cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Set { key: String, value: String },

    Get { key: String },

    Exit,
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

                match cli.command {
                    Commands::Set { key, value } => {
                        println!("SET {} = {}", key, value);
                    }

                    Commands::Get { key } => {
                        println!("GET {}", key);
                    }

                    Commands::Exit => {
                        break;
                    }
                }
            }

            Err(_) => {
                break;
            }
        }
    }
    rl.append_history("history.txt")
}
