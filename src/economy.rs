//! General economy definition

// Base price and price multiplier for CPU
const CPU_BASE_PRICE: f64 = 1.0;
const CPU_PRICE_MULTIPLIER: f64 = 1.5;

// Base price and price multiplier for RAM
const RAM_BASE_PRICE: f64 = 1.0;
const RAM_PRICE_MULTIPLIER: f64 = 1.75;

// Base price and price multiplier for Disk
const DISK_BASE_PRICE: f64 = 1.0;
const DISK_PRICE_MULTIPLIER: f64 = 1.625;

// Perks price
pub const CI_CD_PRICE: f64 = 10.0;
pub const AUTO_SCALING_PRICE: f64 = 50.0;

// Helper to round values to the nearest 0.05
fn round_value(value: f64) -> f64 {
    (value * 20.0).round() / 20.0
}

// Calculate resource upgrade price
pub fn upgrade_price(level: u64, base_price: f64, multiplier: f64) -> f64 {
    round_value(base_price * multiplier.powf(level.saturating_sub(1) as f64))
}

// Calculate CPU price
pub fn cpu_price(level: u64) -> f64 {
    upgrade_price(level, CPU_BASE_PRICE, CPU_PRICE_MULTIPLIER)
}

// Calculate RAM price
pub fn ram_price(level: u64) -> f64 {
    upgrade_price(level, RAM_BASE_PRICE, RAM_PRICE_MULTIPLIER)
}

// Calculate Disk price
pub fn disk_price(level: u64) -> f64 {
    upgrade_price(level, DISK_BASE_PRICE, DISK_PRICE_MULTIPLIER)
}
