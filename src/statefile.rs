//! Create (if needed) and read statefile
//! It contains the resources and perks
//! Also acts as a save file

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::PathBuf;

// Struct for the statefile
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GameState {
    // Resources
    pub cpu: u64,
    pub ram: u64,
    pub disk: u64,
    pub credits: f64,
    // Perks
    pub ci_cd: bool,
    pub auto_scaling: bool,
}

// Default values
impl Default for GameState {
    fn default() -> Self {
        Self {
            // Resources
            cpu: 1,
            ram: 1,
            disk: 1,
            credits: 0.0,
            // Perks
            ci_cd: false,
            auto_scaling: false,
        }
    }
}

// Define the statefile path
fn statefile_path() -> anyhow::Result<PathBuf> {
    let state_dir = dirs::state_dir()
        .context("Failed to determine state directory")?
        .join("incrust");

    fs::create_dir_all(&state_dir).with_context(|| {
        format!(
            "Failed to create the {} state directory",
            state_dir.display()
        )
    })?;

    Ok(state_dir.join("incrust.json"))
}

// Initialize the statefile
pub fn init_statefile() -> anyhow::Result<GameState> {
    let statefile_path = statefile_path()?;

    // If statefile exists, load values from it
    let game_state = if statefile_path.exists() {
        let statefile = File::open(&statefile_path).with_context(|| {
            format!("Failed to open the {} state file", statefile_path.display())
        })?;

        serde_json::from_reader(statefile).with_context(|| {
            format!(
                "Failed to deserialize the {} state file",
                statefile_path.display()
            )
        })?
    // If statefile doesn't exist, load default values
    } else {
        GameState::default()
    };

    // Save / write the state to the statefile,
    // to initialize it if it doesn't exist or to update it if needed
    save(&game_state)?;

    Ok(game_state)
}

// Save (write) the statefile
pub fn save(game_state: &GameState) -> anyhow::Result<()> {
    let statefile_path = statefile_path()?;

    let statefile = File::create(&statefile_path).with_context(|| {
        format!(
            "Failed to create the {} state file",
            statefile_path.display()
        )
    })?;

    serde_json::to_writer_pretty(statefile, game_state).with_context(|| {
        format!(
            "Failed to write the {} state file",
            statefile_path.display()
        )
    })?;

    Ok(())
}
