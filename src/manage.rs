//! Manage menu to enable or disable unlocked perks

use anyhow::Context;
use std::thread;
use std::time::Duration;

use crate::prompt::{self, PromptEditor};
use crate::statefile::{self, GameState};

// Define ID for each perks toggling option
// Perks
const CI_CD_ID: &str = "1";
const AUTO_SCALING_ID: &str = "2";

// Manage menu
pub fn manage_menu(
    game_state: &mut GameState,
    submenu_editor: &mut PromptEditor,
) -> anyhow::Result<()> {
    loop {
        // Manage menu title
        println!("[MANAGE]");
        println!();

        // Perks section
        println!("Perks:");

        // CI/CD
        println!(
            "{CI_CD_ID} - CI/CD         {}",
            if !game_state.ci_cd_unlocked {
                "LOCKED"
            } else if game_state.ci_cd_enabled {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );

        // Auto-Scaling
        println!(
            "{AUTO_SCALING_ID} - Auto-Scaling  {}",
            if !game_state.auto_scaling_unlocked {
                "LOCKED"
            } else if game_state.auto_scaling_enabled {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );

        println!();

        // User guide
        println!("Select the number associated to an unlocked perk to enable or disable it.");
        println!("Type \"exit\" or press \"Ctrl+C\" to exit the manage menu.");

        // Show manage menu prompt and retrieve user selection
        let selection = prompt::show_prompt(submenu_editor, "manage-menu")?;

        // Exit on Ctrl+C
        let Some(selection) = selection else {
            return Ok(());
        };

        // Execute user selection
        match selection.trim() {
            // Perks
            CI_CD_ID => toggle_perk(game_state, CI_CD_ID, "CI/CD")?,
            AUTO_SCALING_ID => toggle_perk(game_state, AUTO_SCALING_ID, "Auto-Scaling")?,

            // Others
            "exit" => return Ok(()),
            _ => println!("Invalid selection"),
        }

        // Formatting and little delay for better UI
        thread::sleep(Duration::from_millis(750));
        println!();
        println!("-----------------------------------");
        println!();
    }
}

// Helper to toggle perks
fn toggle_perk(game_state: &mut GameState, perk_id: &str, perk_name: &str) -> anyhow::Result<()> {
    match perk_id {
        CI_CD_ID => {
            if !game_state.ci_cd_unlocked {
                println!("{perk_name} is not unlocked!");
                return Ok(());
            }

            game_state.ci_cd_enabled = !game_state.ci_cd_enabled;
        }

        AUTO_SCALING_ID => {
            if !game_state.auto_scaling_unlocked {
                println!("{perk_name} is not unlocked!");
                return Ok(());
            }

            game_state.auto_scaling_enabled = !game_state.auto_scaling_enabled;
        }

        _ => return Ok(()),
    }

    statefile::save(game_state).context("Failed to save game")?;

    println!("{perk_name} status successfully updated!");

    Ok(())
}
