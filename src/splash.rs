//! Splash screen

use colored::Colorize;
use std::{thread, time::Duration};

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
        r#"
 ___                           _
|_ _|_ __   ___ _ __ _   _ ___| |_
 | || '_ \ / __| '__| | | / __| __|
 | || | | | (__| |  | |_| \__ \ |_
|___|_| |_|\___|_|   \__,_|___/\__|

    "#
    );
    println!("Version {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("-----------------------------------");
    println!();
    println!("SYSTEM RESOURCES");
    println!();
    println!("CPU        {} CORE(S)", game_state.cpu);
    println!("RAM        {} GB", game_state.ram);
    println!("DISK       {} GB", game_state.disk);
    println!("CREDITS    {}", game_state.credits);
    println!();
    println!("SYSTEM PERKS");
    println!();
    println!(
        "CI/CD      {}",
        if game_state.ci_cd { "TRUE" } else { "FALSE" }
    );
    println!();
    println!("-----------------------------------");
    println!();
    println!("Type 'help' to see available commands");
    println!();
}
