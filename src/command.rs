//! Execute game commands

use colored::Colorize;
use std::{thread, time::Duration};

use crate::statefile::GameState;

pub fn compile() {
    println!("Compiling...");
}

pub fn upgrade() {
    println!("Upgrading...");
}

pub fn status(game_state: &GameState) {
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
}

pub fn guide() {
    println!("Start compiling stuff by running the 'compile' command.");
    println!("You'll earn credits for each completed (and successful) compilation.");
    println!("Compilations have a small chance to fail upon completion due to eventual");
    println!("lack of free space, in which case you won't be rewarded with credits for it.");
    println!("You can interrupt an ongoing compilation before the end by pressing any key,");
    println!("but note that you won't get any credit for it.");
    println!();
    println!("Spend credits to upgrade your system in order to optimize compilation time,");
    println!(
        "reduce chances of compilation failures and increase credits rate (as well as other bonuses)"
    );
    println!("by running the 'upgrade' command:");
    println!();
    println!("- Buying CPU cores speeds up compilation time");
    println!(
        "- Buying RAM increases the number of credits earned per completed (and successful) compilation"
    );
    println!(
        "- Buying Disk space reduces chances of compilation failures due to lack of free space"
    );
    println!("- Other bonuses can be acquired through different one-time purchase system perks");
    println!();
    println!(
        "You can check the current status of your system resources, system perks and credits by running"
    );
    println!("the 'status' command.");
    println!();
    println!("Progression is automatically saved and resumed between game sessions.");
}

pub fn help() {
    println!("Available commands:");
    println!();
    println!("compile  <== Start compiling to earn credits");
    println!("upgrade  <== Upgrade the system to speed up compilation or earn more credits");
    println!("status   <== Check the current status of your system resources and credits");
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
