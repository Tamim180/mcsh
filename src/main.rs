mod commands;
mod completer;
mod highlighter;
mod hinter;

use colored::*;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{ColorMode, CompletionType, Config, Context, Editor, Helper, Result};
use std::borrow::Cow;

use completer::McshCompleter;
use completer::McshPair;
use highlighter::McshHighlighter;
use hinter::{CommandHint, McshHinter};

struct McshHelper {
    completer: McshCompleter,
    highlighter: McshHighlighter,
    hinter: McshHinter,
}

impl Completer for McshHelper {
    type Candidate = McshPair;
    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<McshPair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for McshHelper {
    type Hint = CommandHint;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<CommandHint> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for McshHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("\x1b[2m{}\x1b[0m", hint))
    }
}

impl Validator for McshHelper {}
impl Helper for McshHelper {}

fn print_motd() {
    println!();
    println!("{}", "  ███╗   ███╗ ██████╗███████╗██╗  ██╗".green().bold());
    println!("{}", "  ████╗ ████║██╔════╝██╔════╝██║  ██║".green().bold());
    println!("{}", "  ██╔████╔██║██║     ███████╗███████║".green().bold());
    println!("{}", "  ██║╚██╔╝██║██║     ╚════██║██╔══██║".green().bold());
    println!("{}", "  ██║ ╚═╝ ██║╚██████╗███████║██║  ██║".green().bold());
    println!("{}", "  ╚═╝     ╚═╝ ╚═════╝╚══════╝╚═╝  ╚═╝".green().bold());
    println!();
    println!("{}", "  Minecraft Shell v0.1.0".yellow().bold());
    println!("{}", "  A world where every command starts with /".dimmed());
    println!();
    println!("{}", "  [Server] Welcome to the server!".green());
    println!("{}", "  [Server] Type /help for a list of commands.".green());
    println!("{}", "  [Server] Text without / will be sent as chat.".green());
    println!();
}

fn main() {
    print_motd();

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .color_mode(ColorMode::Enabled)
        .auto_add_history(true)
        .max_history_size(1000)
        .unwrap()
        .build();

    let helper = McshHelper {
        completer: McshCompleter::new(),
        highlighter: McshHighlighter,
        hinter: McshHinter,
    };

    let mut rl: Editor<McshHelper, DefaultHistory> =
        Editor::with_config(config).expect("Failed to create editor");
    rl.set_helper(Some(helper));

    let history_path = get_history_path();
    let _ = rl.load_history(&history_path);

    loop {
        let prompt = commands::get_prompt();

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }

                if line == "/history" {
                    println!("{}", "[Server] Command History:".green());
                    for (i, entry) in rl.history().iter().enumerate() {
                        println!("  {:3}  {}", i + 1, entry.dimmed());
                    }
                    continue;
                }

                let should_continue = commands::handle_command(&line);
                if !should_continue {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "[Server] Connection lost. See you next time!".red());
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(&history_path);
}

fn get_history_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.mcsh_history", home)
}
