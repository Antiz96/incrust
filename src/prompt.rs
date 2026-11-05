//! Show game prompt and return user input

use anyhow::Context;
use colored::Colorize;
use rustyline::completion::{Completer, Pair};
use rustyline::config::CompletionType;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Editor, Helper};
use std::env;

// Auto completion
pub const MAIN_COMMANDS: &[&str] = &[
    "compile", "upgrade", "manage", "status", "guide", "help", "sudo", "hack", "exit",
];

pub const SUBMENU_COMMANDS: &[&str] = &["exit"];

pub struct CommandCompleter {
    commands: Vec<&'static str>,
}

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let input = &line[..pos];

        let matches = self
            .commands
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
pub fn create_rl_editor(auto_complete_commands: Vec<&'static str>) -> anyhow::Result<PromptEditor> {
    let config = rustyline::Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl_editor =
        Editor::with_config(config).context("Failed to initialize command prompt")?;

    rl_editor.set_helper(Some(CommandCompleter {
        commands: auto_complete_commands,
    }));

    Ok(rl_editor)
}

// Game prompt
pub fn show_prompt(
    rl_editor: &mut PromptEditor,
    prompt_context: &str,
) -> anyhow::Result<Option<String>> {
    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());

    let prompt = format!(
        "{}{}{}{}{}{}",
        "[".yellow(),
        username.blue(),
        "@".yellow(),
        match prompt_context {
            "incrust" => prompt_context.blue(),
            "upgrade-shop" => prompt_context.green(),
            "manage-menu" => prompt_context.red(),
            _ => prompt_context.normal(),
        },
        " ~]".yellow(),
        "$ ".yellow()
    );

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
