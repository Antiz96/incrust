//! Start the fake compilation process

use anyhow::Context;
use colored::Colorize;
use rand::random;
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use which::which;

use crate::command;
use crate::parameters;
use crate::statefile::{self, GameState};
use crate::upgrade;

pub fn compile_process(
    game_state: &mut GameState,
    interrupt_rx: &Receiver<()>,
) -> anyhow::Result<bool> {
    // Run auto-scaling if enabled
    upgrade::auto_scale(game_state)?;

    // Get compilation parameters
    let (compile_time, credit_reward, failure_probability) =
        parameters::get_compilation_parameters(game_state);

    // Show status (to see the current numbers of resources and compilation parameters)
    command::status(game_state);
    println!();

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

    // Convert compile time to a Duration (used later to determine when the compilation is over)
    let compile_time = Duration::from_secs_f64(compile_time);

    // Start compilation timer
    let start = Instant::now();

    // Start genact (if it's installed, directly checked by the start_genact function)
    let mut genact_process = start_genact(game_state);

    // Fallback compilation animation if genact is not installed
    let spinner = ["-", "\\", "|", "/"];
    let mut spinner_index = 0;

    // Main compilation loop
    loop {
        // Check for interruption request via Ctrl+C
        match interrupt_rx.try_recv() {
            Ok(()) => {
                // Stop genact (if needed)
                stop_genact(&mut genact_process);

                println!();
                println!(
                    "{} Compilation interrupted, no credits earned...",
                    "[INFO]".blue()
                );
                return Ok(true);
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                // Stop genact (if needed)
                stop_genact(&mut genact_process);
                eprintln!("Warning: Interruption handler disconnected.");
                return Ok(true);
            }
        }

        // Check whether compilation is complete
        if start.elapsed() >= compile_time {
            break;
        }

        // Display fallback animation if genact isn't running / installed
        if genact_process.is_none() {
            print!("\rCompiling {}", spinner[spinner_index]);
            let _ = io::stdout().flush();

            spinner_index = (spinner_index + 1) % spinner.len();
        }

        // Wait before checking restarting the loop
        thread::sleep(Duration::from_millis(100));
    }

    // Stop genact (if needed)
    stop_genact(&mut genact_process);

    // Move to a new line after the animation
    println!();

    // Compilation completion message
    println!("Compilation completed!");
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

        println!("{} You earned {credit_reward:.2} credits!", "[INFO]".blue());
    }

    Ok(false)
}

// Helper to start genact for compilation animation (if it's installed)
fn start_genact(game_state: &GameState) -> Option<Child> {
    // Check if genact is installed
    let genact_path = which("genact").ok()?;

    // Calculate speed factor according to the number of CPU cores
    let speed_factor = game_state.cpu.div_ceil(10);

    // Run genact
    Command::new(genact_path)
        .arg("--speed-factor")
        .arg(speed_factor.to_string())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .ok()
}

// Helper to stop genact (if there is a genact process running)
fn stop_genact(genact_process: &mut Option<Child>) {
    // Kill the running genact process spawned by incrust (if there's one)
    if let Some(mut child) = genact_process.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
}
