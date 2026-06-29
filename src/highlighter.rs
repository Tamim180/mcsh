use rustyline::highlight::Highlighter;
use std::borrow::Cow;

use crate::commands::ALL_COMMANDS;

pub struct McshHighlighter;

impl Highlighter for McshHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if line.is_empty() {
            return Cow::Borrowed(line);
        }

        // Chat message (no slash) - show dimmed
        if !line.starts_with('/') {
            return Cow::Owned(format!("\x1b[2m{}\x1b[0m", line));
        }

        // Split into command and args
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let cmd = parts[0];
        let args = if parts.len() > 1 { parts[1] } else { "" };

        // Check if it's a known command
        let is_known = ALL_COMMANDS.iter().any(|c| c.eq_ignore_ascii_case(cmd));

        let colored_cmd = if is_known {
            // Known command: gold/yellow
            format!("\x1b[33;1m{}\x1b[0m", cmd)
        } else if cmd.len() > 1 {
            // Unknown /something: red
            format!("\x1b[31m{}\x1b[0m", cmd)
        } else {
            // Just a slash
            format!("\x1b[33m{}\x1b[0m", cmd)
        };

        if args.is_empty() {
            Cow::Owned(colored_cmd)
        } else {
            // Args: cyan
            Cow::Owned(format!("{} \x1b[36m{}\x1b[0m", colored_cmd, args))
        }
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        true
    }
}
