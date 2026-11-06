//! Start the fake compilation process

use colored::Colorize;
use std::io::{self, Write};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use crate::parameters;
use crate::statefile::{self, GameState};

pub fn compile_proc(
    game_state: &mut GameState,
    interrupt_rx: &Receiver<()>,
) -> anyhow::Result<bool> {
    // Get compilation parameters
    let compile_time =
        Duration::from_secs_f64(parameters::compile_time(game_state.cpu, game_state.ram));
    let credit_reward = parameters::credit_reward(game_state.ram, game_state.disk);
    let failure_probability = parameters::failure_probability(game_state.cpu, game_state.disk);

    // Set timestamps to calculate compilation remaining time
    let start = Instant::now();
    let mut last_remaining = u64::MAX;

    // Silly messages
    println!(
        "{} Synchronizing imaginary dependencies...",
        "[ OK ]".green()
    );
    thread::sleep(Duration::from_millis(125));

    println!(
        "{} Flushing the cache of past build failures...",
        "[ OK ]".green()
    );
    thread::sleep(Duration::from_millis(125));

    println!("{} Starting compilation...", "[ OK ]".green());
    thread::sleep(Duration::from_millis(125));

    println!();

    // Show compilation parameters
    parameters::show_compilation_parameters(game_state);
    println!();

    // Main compilation loop
    loop {
        // Check for interruption request via Ctrl+C
        match interrupt_rx.try_recv() {
            Ok(()) => {
                println!();
                println!(
                    "\n{} Compilation interrupted, no credits earned...",
                    "[INFO]".blue()
                );
                return Ok(true);
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                eprintln!("\nWarning: Interruption handler disconnected.");
                return Ok(true);
            }
        }

        // Calculate remaining time
        let remaining = compile_time.saturating_sub(start.elapsed());

        // Check whether compilation is complete
        if remaining.is_zero() {
            break;
        }

        // Display countdown
        let remaining_secs = remaining.as_secs_f64().ceil() as u64;

        if remaining_secs != last_remaining {
            let minutes = remaining_secs / 60;
            let seconds = remaining_secs % 60;

            print!("\rCompiling... {minutes:02}:{seconds:02} remaining ");
            let _ = io::stdout().flush();

            last_remaining = remaining_secs;
        }

        // Wait before checking again
        thread::sleep(Duration::from_millis(100));
    }

    // Compilation completion message
    println!("\rCompilation completed!                     ");
    println!();

    // Roll for compilation failure
    if rand::random::<f64>() < failure_probability {
        // Failure message
        println!(
            "{} Build failed: insufficient disk space...",
            "[FAIL]".red()
        );
        thread::sleep(Duration::from_millis(125));

        println!(
            "{} Cleaning up old build chroots to free up space for next attempts...",
            "[INFO]".blue()
        );
    } else {
        // Award credits and save state
        game_state.credits += credit_reward;
        statefile::save(game_state)?;

        // Success message
        println!("{} Compilation successful!", "[INFO]".blue());
        thread::sleep(Duration::from_millis(125));

        println!("{} You earned {credit_reward} credits!", "[INFO]".blue());
    }

    Ok(false)
}
