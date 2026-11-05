//! Show game prompt and return user input

use anyhow::Context;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Editor, Helper};
use std::env;

// Auto completion
const COMMANDS: &[&str] = &[
    "compile", "upgrade", "status", "guide", "help", "sudo", "hack", "exit",
];

pub struct CommandCompleter;

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let input = &line[..pos];

        let matches = COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(input))
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: format!("{cmd} "),
            })
            .collect();

        Ok((0, matches))
    }
}

impl Hinter for CommandCompleter {
    type Hint = String;
}

impl Highlighter for CommandCompleter {}
impl Validator for CommandCompleter {}
impl Helper for CommandCompleter {}

pub type PromptEditor = Editor<CommandCompleter, DefaultHistory>;

// Create rustyline editor
pub fn create_editor() -> anyhow::Result<PromptEditor> {
    let config = rustyline::Config::builder()
        .completion_type(rustyline::config::CompletionType::List)
        .build();

    let mut editor = Editor::with_config(config).context("Failed to initialize command prompt")?;
    editor.set_helper(Some(CommandCompleter));

    Ok(editor)
}

// Game prompt
pub fn show_prompt(rl_editor: &mut PromptEditor) -> anyhow::Result<Option<String>> {
    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let prompt = format!("[{username}@incrust ~]$ ");

    match rl_editor.readline(&prompt) {
        Ok(command) => {
            if !command.trim().is_empty() {
                let _ = rl_editor.add_history_entry(&command);
            }

            Ok(Some(command))
        }
        Err(ReadlineError::Interrupted) => Ok(None),
        Err(ReadlineError::Eof) => Ok(Some("exit".to_string())),
        Err(error) => Err(error).context("Failed to read stdin"),
    }
}
