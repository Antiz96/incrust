//! Execute game commands

use colored::Colorize;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use crate::compile;
use crate::parameters;
use crate::statefile::GameState;

pub fn compile(game_state: &mut GameState, interrupt_rx: &Receiver<()>) -> anyhow::Result<()> {
    // Discard pending interruption signals
    while interrupt_rx.try_recv().is_ok() {}

    // Start the fake compile process and loop over it
    // Break the loop if it gets interrupted (e.g. if an error occurred or if the user pressed Ctrl+C)
    // or if the CI/CD perk is not unlocked yet
    loop {
        let interrupted = compile::compile_proc(game_state, interrupt_rx)?;

        if interrupted || !game_state.ci_cd {
            break;
        }
        println!();
    }

    Ok(())
}

pub fn upgrade() {
    println!("Upgrading...");
}

pub fn status(game_state: &GameState) {
    println!("[RESOURCES]");
    println!();
    println!("CPU            {} CORE(S)", game_state.cpu);
    println!("RAM            {} GB", game_state.ram);
    println!("DISK           {} GB", game_state.disk);
    println!("CREDITS        {}", game_state.credits);
    println!();
    println!("[PERKS]");
    println!();
    println!(
        "CI/CD          {}",
        if game_state.ci_cd { "TRUE" } else { "FALSE" }
    );
    println!(
        "Auto-Scaling   {}",
        if game_state.auto_scaling {
            "TRUE"
        } else {
            "FALSE"
        }
    );
    println!();
    println!("[COMPILATION]");
    println!();
    parameters::show_compilation_parameters(game_state);
}

pub fn guide() {
    println!("Start compiling stuff by running the 'compile' command.");
    println!("You'll earn credits for each completed (and successful) compilation.");
    println!("Compilations have a small chance to fail upon completion, in which case");
    println!("you won't get rewarded with credits for it.");
    println!("Ongoing compilation can be interrupted by pressing Ctrl+C, but note that");
    println!("you won't get any credit for it either.");
    println!();
    println!("Spend credits to upgrade your system resources and buy different perks by running");
    println!("the 'upgrade' command:");
    println!();
    println!("- Buying CPU cores speeds up compile time");
    println!("  but also slightly increases chances of compilation failures");
    println!("- Buying RAM increases the credit reward per completed (and successful) compilation");
    println!("  but also slightly increases the compile time");
    println!("- Buying Disk space reduces chances of compilation failures");
    println!(
        "  but also slightly reduces the credit reward per completed (and successful) compilation"
    );
    println!("- Other bonuses can be acquired through different one-time purchase perks");
    println!();
    println!(
        "You can check the current status of your resources and perks by running the 'status' command."
    );
    println!();
    println!("Progression is automatically saved and resumed between game sessions.");
}

pub fn help() {
    println!("Available commands:");
    println!();
    println!("compile  <== Start compiling to earn credits");
    println!("upgrade  <== Upgrade the system to speed up compilation or earn more credits");
    println!("status   <== Check the current status of your resources and perks");
    println!("guide    <== Show the game guide");
    println!("help     <== Show this message");
    println!("exit     <== Save and exit the game");
}

pub fn sudo() {
    println!("You cannot administrate this nonsense!");
    println!("This incident will be reported.");
}

pub fn hack() {
    println!("Access granted");
}

pub fn exit() {
    println!("{} Terminating background nonsense...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Saving imaginary progress...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Releasing precious resources...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(500));

    println!("{} Returning borrowed brain cells...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(500));

    println!("{} Putting CPU to bed...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Considering touching some grass...", "[FAIL]".red());
    thread::sleep(Duration::from_millis(250));

    println!("{} Flushing coffee reserves...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(1500));

    println!("{} Removing temporary files...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Unmounting virtual filesystems...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(250));

    println!("{} Shutting down system...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(1000));
}

pub fn invalid() {
    eprintln!("Invalid command");
    eprintln!("Type 'help' to see available commands");
}
