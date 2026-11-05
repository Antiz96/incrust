//! A minimal incremental / idle terminal game about compiling stuff

use clap::Parser;
use std::process;
use std::sync::mpsc;

mod command;
mod compile;
mod economy;
mod help;
mod lockfile;
mod manage;
mod parameters;
mod prompt;
mod splash;
mod statefile;
mod upgrade;
mod version;

use crate::prompt::MAIN_COMMANDS;
use crate::prompt::SUBMENU_COMMANDS;

// Arguments definition
#[derive(Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[arg(short = 'h', long)]
    help: bool,

    #[arg(short = 'V', long)]
    version: bool,
}

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Show help message if the -h / --help arg is passed
    if args.help {
        help::show_help();
        return;
    }

    // Show name and version if the -V / --version arg is passed
    if args.version {
        version::show_version();
        return;
    }

    // Create (if needed) and acquire lockfile
    // Exit if there's already an instance running
    // or if there was an issue creating or acquiring the lockfile (e.g. permission issue)
    let _lockfile = lockfile::acquire_lockfile().unwrap_or_else(|error| {
        eprintln!("{error:?}");
        process::exit(1);
    });

    // Initialize / update statefile and load state value
    // Exit if there was an issue when creating, reading, deserializing or writing the statefile
    let mut game_state = statefile::init_statefile().unwrap_or_else(|error| {
        eprintln!("{error:?}");
        process::exit(2);
    });

    // Show splash screen
    splash::splash_screen(&game_state);

    // Initialize rustyline editors
    // Used to handle usual keyboard shortcuts within the game prompt
    // like arrow keys to navigate through commands history and move the cursor,
    // Ctrl+D to quit / disconnect, etc...
    // We have one editor dedicated to the main game prompt and one for the submenus prompt,
    // so they get a separate PS1 and command history
    // Exit if there's an error when initializing the editor
    let mut main_editor =
        prompt::create_rl_editor(MAIN_COMMANDS.to_vec()).unwrap_or_else(|error| {
            eprintln!("{error:?}");
            process::exit(3);
        });

    let mut submenu_editor =
        prompt::create_rl_editor(SUBMENU_COMMANDS.to_vec()).unwrap_or_else(|error| {
            eprintln!("{error:?}");
            process::exit(3);
        });

    // Create interruption channel and register the Ctrl+C handler
    // Used to allow interrupting ongoing compilations by pressing Ctrl+C
    let (interrupt_tx, interrupt_rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        interrupt_tx.send(()).unwrap_or_else(|error| {
            eprintln!("Failed to send interruption signal:\n{error}");
            process::exit(4);
        })
    })
    .unwrap_or_else(|error| {
        eprintln!("Failed to register Ctrl+C handler:\n{error}");
        process::exit(4);
    });

    // Main game loop
    loop {
        // Show game prompt and retrieve command
        // Exit if there was an issue when reading stdin
        let command = prompt::show_prompt(&mut main_editor, "incrust").unwrap_or_else(|error| {
            eprintln!("{error:?}");
            process::exit(5);
        });

        // Allow empty command (handled below, just re-print the prompt)
        let Some(command) = command else {
            continue;
        };

        // Execute command
        match command.trim() {
            "compile" => command::compile(&mut game_state, &interrupt_rx).unwrap_or_else(|error| {
                eprintln!("{error:?}");
                process::exit(2);
            }),
            "upgrade" => {
                command::upgrade(&mut game_state, &mut submenu_editor).unwrap_or_else(|error| {
                    eprintln!("{error:?}");
                    process::exit(2);
                })
            }
            "manage" => {
                command::manage(&mut game_state, &mut submenu_editor).unwrap_or_else(|error| {
                    eprintln!("{error:?}");
                    process::exit(2);
                })
            }
            "status" => command::status(&game_state),
            "guide" => command::guide(),
            "help" => command::help(),
            "sudo" => command::sudo(),
            "hack" => command::hack(),
            "exit" => {
                command::exit();
                break;
            }
            "" => (),
            _ => command::invalid(),
        }
    }
}
