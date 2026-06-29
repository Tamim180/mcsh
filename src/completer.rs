use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::Context;
use std::env;
use std::fs;

use crate::commands::ALL_COMMANDS;

pub type McshPair = Pair;

pub struct McshCompleter {
    _file_completer: FilenameCompleter,
}

impl McshCompleter {
    pub fn new() -> Self {
        McshCompleter {
            _file_completer: FilenameCompleter::new(),
        }
    }
}

impl Completer for McshCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let line_up_to_cursor = &line[..pos];

        // Completing a /command (no space yet)
        if !line_up_to_cursor.contains(' ') {
            if line_up_to_cursor.starts_with('/') {
                let prefix = line_up_to_cursor.to_lowercase();
                let candidates: Vec<Pair> = ALL_COMMANDS
                    .iter()
                    .filter(|cmd| cmd.starts_with(prefix.as_str()))
                    .map(|cmd| Pair {
                        display: cmd.to_string(),
                        replacement: format!("{} ", cmd),
                    })
                    .collect();
                return Ok((0, candidates));
            }
            return Ok((pos, vec![]));
        }

        // Completing file path arguments
        let last_space = line_up_to_cursor.rfind(' ').map(|i| i + 1).unwrap_or(0);
        let arg_prefix = &line_up_to_cursor[last_space..];

        let mut candidates: Vec<Pair> = vec![];

        let (dir_part, file_prefix) = if let Some(slash) = arg_prefix.rfind('/') {
            (&arg_prefix[..=slash], &arg_prefix[slash + 1..])
        } else {
            (".", arg_prefix)
        };

        let search_dir = if dir_part == "." {
            env::current_dir().unwrap_or_default()
        } else {
            std::path::PathBuf::from(dir_part)
        };

        if let Ok(entries) = fs::read_dir(&search_dir) {
            let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            items.sort_by_key(|e| e.file_name());
            for entry in items {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with(file_prefix) {
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    let replacement = if dir_part == "." {
                        if is_dir {
                            format!("{}/", name_str)
                        } else {
                            name_str.to_string()
                        }
                    } else {
                        if is_dir {
                            format!("{}{}/", dir_part, name_str)
                        } else {
                            format!("{}{}", dir_part, name_str)
                        }
                    };
                    candidates.push(Pair {
                        display: name_str.to_string(),
                        replacement,
                    });
                }
            }
        }

        Ok((last_space, candidates))
    }
}
