//! General parameters definition

use crate::statefile::GameState;

// Base and minimum compile time (in seconds)
const BASE_COMPILE_TIME: f64 = 300.0;
const MIN_COMPILE_TIME: f64 = 10.0;

// Base and minimum credit reward for a successful compilation
const BASE_CREDIT_REWARD: f64 = 1.0;
const MIN_CREDIT_REWARD: f64 = 0.5;

// Base and minimum / maximum probability of compilation failure
const BASE_COMPILATION_FAILURE_PROB: f64 = 0.25;
const MIN_COMPILATION_FAILURE_PROB: f64 = 0.05;
const MAX_COMPILATION_FAILURE_PROB: f64 = 0.90;

// Base resource requirement and increment for prestige levels
const BASE_PRESTIGE_RESOURCE_REQUIREMENT: u64 = 100;
const PRESTIGE_RESOURCE_INCREMENT: u64 = 20;

// Credit reward bonus percentage for each prestige levels
const PRESTIGE_CREDIT_REWARD_BONUS: f64 = 0.20;

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
// - Each prestige levels increases the default reward by 20%
// - Minimum possible credit reward is capped at `MIN_CREDIT_REWARD`
pub fn credit_reward(ram: u64, disk: u64, prestige: u64) -> f64 {
    let reward = BASE_CREDIT_REWARD + 0.5 * (ram.saturating_sub(1) as f64)
        - 0.1 * (disk.saturating_sub(1) as f64);

    let prestige_bonus = 1.0 + PRESTIGE_CREDIT_REWARD_BONUS * prestige as f64;

    reward.max(MIN_CREDIT_REWARD) * prestige_bonus
}

// Calculate compilation failure probability based on CPU and Disk
// - Each additional GB of Disk reduces the probability of compilation failure by 1%
// - Each additional CPU core increases the probability of compilation failure by 0.75%
// - Minimum probability is capped at `MIN_COMPILATION_FAILURE_PROB`
// - Maximum probability is capped at `MAX_COMPILATION_FAILURE_PROB`
pub fn failure_probability(cpu: u64, disk: u64) -> f64 {
    let probability = BASE_COMPILATION_FAILURE_PROB - 0.010 * (disk.saturating_sub(1) as f64)
        + 0.0075 * (cpu.saturating_sub(1) as f64);

    probability.clamp(MIN_COMPILATION_FAILURE_PROB, MAX_COMPILATION_FAILURE_PROB)
}

// Calculate prestige resource requirement based on prestige level
pub fn prestige_resource_requirement(prestige: u64) -> u64 {
    BASE_PRESTIGE_RESOURCE_REQUIREMENT + PRESTIGE_RESOURCE_INCREMENT * prestige
}

// Helper to retrieve the different compilation parameters:
// compile time, credit reward and failure percentage
pub fn get_compilation_parameters(game_state: &GameState) -> (f64, f64, f64) {
    // Calculate values
    let compile_time = compile_time(game_state.cpu, game_state.ram);
    let credit_reward = credit_reward(game_state.ram, game_state.disk, game_state.prestige);
    let failure_probability = failure_probability(game_state.cpu, game_state.disk);

    // Return values
    (compile_time, credit_reward, failure_probability)
}
