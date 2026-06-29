use rustyline::hint::{Hint, Hinter};
use rustyline::Context;

use crate::commands::ALL_COMMANDS;

pub struct McshHinter;

#[derive(Clone, Debug)]
pub struct CommandHint(String);

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.0
    }

    fn completion(&self) -> Option<&str> {
        Some(&self.0)
    }
}

impl Hinter for McshHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        // Only hint at command level (no space yet)
        if !line.contains(' ') && line.starts_with('/') {
            let matches: Vec<&&str> = ALL_COMMANDS
                .iter()
                .filter(|cmd| cmd.starts_with(line) && **cmd != line)
                .collect();

            if matches.len() == 1 {
                let hint_text = &matches[0][line.len()..];
                return Some(CommandHint(hint_text.to_string()));
            }
        }

        None
    }
}
