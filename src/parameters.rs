//! General parameters definition

use crate::statefile::GameState;

// Base and minimum compile time (in seconds)
const BASE_COMPILE_TIME: f64 = 600.0;
const MIN_COMPILE_TIME: f64 = 10.0;

// Base and minimum credit reward for a successful compilation
const BASE_CREDIT_REWARD: f64 = 1.0;
const MIN_CREDIT_REWARD: f64 = 1.0;

// Base and minimum / maximum probability of compilation failure
const BASE_COMPILATION_FAILURE_PROB: f64 = 0.20;
const MIN_COMPILATION_FAILURE_PROB: f64 = 0.10;
const MAX_COMPILATION_FAILURE_PROB: f64 = 0.90;

// Calculate compile time based on CPU and RAM
// - Each additional CPU core reduces the compile time by 5 seconds
// - Each additional GB of RAM increases the compile time by 2.5 seconds
// - Minimum possible compile time is capped at `MIN_COMPILE_TIME`
pub fn compile_time(cpu: u64, ram: u64) -> f64 {
    let time = BASE_COMPILE_TIME - 5.0 * (cpu.saturating_sub(1) as f64)
        + 2.5 * (ram.saturating_sub(1) as f64);

    time.max(MIN_COMPILE_TIME)
}

// Calculate the credit reward after a successful compilation based on RAM and Disk
// - Each additional GB of RAM increases the number of credits earned by 0.5
// - Each additional GB of Disk reduces the number of credits earned by 0.25
// - Minimum possible credit reward is capped at `MIN_CREDIT_REWARD`
pub fn credit_reward(ram: u64, disk: u64) -> f64 {
    let reward = BASE_CREDIT_REWARD + 0.5 * (ram.saturating_sub(1) as f64)
        - 0.25 * (disk.saturating_sub(1) as f64);

    reward.max(MIN_CREDIT_REWARD)
}

// Calculate compilation failure probability based on CPU and Disk
// - Each additional CPU core increases the probability of compilation failure by 0.025
// - Each additional GB of Disk reduces the probability of compilation failure by 0.05
// - Minimum probability is capped at `MIN_COMPILATION_FAILURE_PROB`
// - Maximum probability is capped at `MAX_COMPILATION_FAILURE_PROB`
pub fn failure_probability(cpu: u64, disk: u64) -> f64 {
    let probability = BASE_COMPILATION_FAILURE_PROB + 0.025 * (cpu.saturating_sub(1) as f64)
        - 0.05 * (disk.saturating_sub(1) as f64);

    probability.clamp(MIN_COMPILATION_FAILURE_PROB, MAX_COMPILATION_FAILURE_PROB)
}

// Helper to show the different compilation parameters (compile time, credit reward and failure
// percentage)
pub fn show_compilation_parameters(game_state: &GameState) {
    let compile_time = compile_time(game_state.cpu, game_state.ram);
    let credit_reward = credit_reward(game_state.ram, game_state.disk);
    let failure_probability = failure_probability(game_state.cpu, game_state.disk);

    println!(
        "Compile time   {:02}:{:02}",
        (compile_time / 60.0) as u64,
        compile_time as u64 % 60
    );
    println!("Credit reward  {credit_reward}");
    println!("Failure        {:.1}%", failure_probability * 100.0);
}
