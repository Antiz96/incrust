//! General parameters definition

use crate::statefile::GameState;

// Base and minimum compile time (in seconds)
const BASE_COMPILE_TIME: f64 = 300.0;
const MIN_COMPILE_TIME: f64 = 5.0;

// Base and minimum credit reward for a successful compilation
const BASE_CREDIT_REWARD: f64 = 1.0;
const MIN_CREDIT_REWARD: f64 = 0.5;

// Base and minimum / maximum probability of compilation failure
const BASE_COMPILATION_FAILURE_PROB: f64 = 0.30;
const MIN_COMPILATION_FAILURE_PROB: f64 = 0.05;
const MAX_COMPILATION_FAILURE_PROB: f64 = 0.90;

// Calculate compile time based on CPU and RAM
// - Each additional CPU core reduces the compile time by 5 seconds
// - Each additional GB of RAM increases the compile time by 1 second
// - Minimum possible compile time is capped at `MIN_COMPILE_TIME`
pub fn compile_time(cpu: u64, ram: u64) -> f64 {
    let time = BASE_COMPILE_TIME - 5.0 * (cpu.saturating_sub(1) as f64)
        + 1.0 * (ram.saturating_sub(1) as f64);

    time.max(MIN_COMPILE_TIME)
}

// Calculate the credit reward after a successful compilation based on RAM and Disk
// - Each additional GB of RAM increases the number of credits earned by 0.5
// - Each additional GB of Disk reduces the number of credits earned by 0.1
// - Minimum possible credit reward is capped at `MIN_CREDIT_REWARD`
pub fn credit_reward(ram: u64, disk: u64) -> f64 {
    let reward = BASE_CREDIT_REWARD + 0.5 * (ram.saturating_sub(1) as f64)
        - 0.1 * (disk.saturating_sub(1) as f64);

    reward.max(MIN_CREDIT_REWARD)
}

// Calculate compilation failure probability based on CPU and Disk
// - Each additional GB of Disk reduces the probability of compilation failure by 1.5%
// - Each additional CPU core increases the probability of compilation failure by 1%
// - Minimum probability is capped at `MIN_COMPILATION_FAILURE_PROB`
// - Maximum probability is capped at `MAX_COMPILATION_FAILURE_PROB`
pub fn failure_probability(cpu: u64, disk: u64) -> f64 {
    let probability = BASE_COMPILATION_FAILURE_PROB - 0.015 * (disk.saturating_sub(1) as f64)
        + 0.010 * (cpu.saturating_sub(1) as f64);

    probability.clamp(MIN_COMPILATION_FAILURE_PROB, MAX_COMPILATION_FAILURE_PROB)
}

// Helper to retrieve and show the different compilation parameters:
// compile time, credit reward and failure percentage
pub fn show_compilation_parameters(game_state: &GameState) -> (f64, f64, f64) {
    // Calculate values
    let compile_time = compile_time(game_state.cpu, game_state.ram);
    let credit_reward = credit_reward(game_state.ram, game_state.disk);
    let failure_probability = failure_probability(game_state.cpu, game_state.disk);

    // Show values
    println!(
        "Compile time         {:02}:{:02}",
        (compile_time / 60.0) as u64,
        compile_time as u64 % 60
    );
    println!("Credit reward        {credit_reward:.2}");
    println!("Failure probability  {:.1}%", failure_probability * 100.0);

    // Return values
    (compile_time, credit_reward, failure_probability)
}
