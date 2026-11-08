//! Start the fake compilation process

use anyhow::Context;
use colored::Colorize;
use rand::random;
use std::io::{self, Write};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use crate::parameters;
use crate::statefile::{self, GameState};
use crate::upgrade;

pub fn compile_proc(
    game_state: &mut GameState,
    interrupt_rx: &Receiver<()>,
) -> anyhow::Result<bool> {
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

    // Run auto-scaling if enabled
    upgrade::auto_scale(game_state)?;

    // Get and show compilation parameters
    let (compile_time, credit_reward, failure_probability) =
        parameters::show_compilation_parameters(game_state);
    println!();

    // Convert compile time to a Duration (used later to calculate remaining time)
    let compile_time = Duration::from_secs_f64(compile_time);

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
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
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
    if random::<f64>() < failure_probability {
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
        game_state.credits = (game_state.credits * 100.0).round() / 100.0;
        statefile::save(game_state).context("Failed to save game")?;

        // Success message
        println!("{} Compilation successful!", "[INFO]".blue());
        thread::sleep(Duration::from_millis(125));

        println!("{} You earned {credit_reward} credits!", "[INFO]".blue());
    }

    Ok(false)
}
