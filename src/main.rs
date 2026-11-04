//! A minimal incremental / idle terminal game about compiling stuff

use clap::Parser;
use std::process;

mod command;
mod help;
mod lockfile;
mod menu;
mod splash;
mod statefile;
mod version;

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
    // Exit on error
    let game_state = statefile::init_statefile().unwrap_or_else(|error| {
        eprintln!("{error:?}");
        process::exit(2);
    });

    // Show splash screen
    splash::splash_screen(&game_state);

    // Main game loop
    loop {
        // Show game menu and execute command
        match menu::show_menu().trim() {
            "compile" => command::compile(),
            "upgrade" => command::upgrade(),
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
