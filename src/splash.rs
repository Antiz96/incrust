//! Splash screen

use colored::Colorize;
use std::thread;
use std::time::Duration;

use crate::command;
use crate::statefile::GameState;

pub fn splash_screen(game_state: &GameState) {
    println!("{} Starting system initialization...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Mounting virtual filesystems...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Initializing kernel subsystems...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(500));

    println!("{} Probing hardware capabilities...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(500));

    println!("{} Detecting available brain cells...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Allocating precious resources...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Fetching coffee...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(1500));

    println!("{} Warming up the CPU...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Checking if it works on my machine...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Loading essential nonsense...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(1000));

    println!(
        "{}",
        r#"
 ___                           _
|_ _|_ __   ___ _ __ _   _ ___| |_
 | || '_ \ / __| '__| | | / __| __|
 | || | | | (__| |  | |_| \__ \ |_
|___|_| |_|\___|_|   \__,_|___/\__|

        "#
        .blue()
    );
    println!("Version {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("-----------------------------------");
    println!();
    command::status(game_state);
    println!();
    println!("-----------------------------------");
    println!();
    println!("Type 'help' to see available commands");
    println!();
}
