//! Shop menu to buy system upgrades and perks

use anyhow::Context;
use std::thread;
use std::time::Duration;

use crate::economy;
use crate::prompt::{self, PromptEditor};
use crate::statefile::{self, GameState};

// Define ID for each upgrade option
// Resources
const CPU_ID: &str = "1";
const RAM_ID: &str = "2";
const DISK_ID: &str = "3";
// Perks
const CI_CD_ID: &str = "4";
const AUTO_SCALING_ID: &str = "5";

// Upgrade shop
pub fn upgrade_shop(
    game_state: &mut GameState,
    submenu_editor: &mut PromptEditor,
) -> anyhow::Result<()> {
    loop {
        // Upgrade shop title and credits status
        println!("[UPGRADE]");
        println!();
        println!("Credits: {:.2}", game_state.credits);
        println!();

        // Resources section
        println!("Resources:");

        // CPU
        println!(
            "{CPU_ID} - CPU   {} -> {}   {:.2} credits",
            game_state.cpu,
            game_state.cpu + 1,
            economy::cpu_price(game_state.cpu + 1),
        );
        println!(
            "    Reduces the compile time but slightly increases chances of compilation failures."
        );

        // RAM
        println!(
            "{RAM_ID} - RAM   {} -> {}   {:.2} credits",
            game_state.ram,
            game_state.ram + 1,
            economy::ram_price(game_state.ram + 1),
        );
        println!(
            "    Increases the credit reward per compilation but slightly increases the compile time."
        );

        // Disk
        println!(
            "{DISK_ID} - Disk  {} -> {}   {:.2} credits",
            game_state.disk,
            game_state.disk + 1,
            economy::disk_price(game_state.disk + 1),
        );
        println!(
            "    Reduces chances of compilation failures but slightly reduces the credit reward per compilation."
        );

        println!();

        // Perks section
        println!("Perks:");

        // CI/CD
        println!(
            "{CI_CD_ID} - CI/CD          {:.2} credits",
            economy::CI_CD_PRICE
        );
        println!(
            "    {}",
            if game_state.ci_cd_unlocked {
                "Already unlocked"
            } else {
                "Automatically restarts a new compilation when the current one ends."
            }
        );

        // Auto-Scaling
        println!(
            "{AUTO_SCALING_ID} - Auto-Scalling  {:.2} credits",
            economy::AUTO_SCALING_PRICE
        );
        println!(
            "    {}",
            if game_state.auto_scaling_unlocked {
                "Already unlocked"
            } else {
                "Automatically buy affordable resource upgrades when starting a compilation."
            }
        );

        println!();

        // User guide
        println!("Select the number associated to the resource or perk you want to buy.");
        println!("Type \"exit\" or press \"Ctrl+C\" to exit the upgrade shop.");

        // Show upgrade shop prompt and retrieve user selection
        let selection = prompt::show_prompt(submenu_editor, "upgrade-shop")?;

        // Exit on Ctrl+C
        let Some(selection) = selection else {
            return Ok(());
        };

        // Execute user selection
        match selection.trim() {
            // Resources
            CPU_ID => upgrade_resource(
                game_state,
                CPU_ID,
                "CPU",
                economy::cpu_price(game_state.cpu + 1),
                false,
            )?,
            RAM_ID => upgrade_resource(
                game_state,
                RAM_ID,
                "RAM",
                economy::ram_price(game_state.ram + 1),
                false,
            )?,
            DISK_ID => upgrade_resource(
                game_state,
                DISK_ID,
                "Disk",
                economy::disk_price(game_state.disk + 1),
                false,
            )?,

            // Perks
            CI_CD_ID => buy_perk(game_state, CI_CD_ID, "CI/CD", economy::CI_CD_PRICE)?,
            AUTO_SCALING_ID => buy_perk(
                game_state,
                AUTO_SCALING_ID,
                "Auto-Scaling",
                economy::AUTO_SCALING_PRICE,
            )?,

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

// Helper to upgrade resources
fn upgrade_resource(
    game_state: &mut GameState,
    resource_id: &str,
    resource_name: &str,
    price: f64,
    auto_scale: bool,
) -> anyhow::Result<()> {
    if game_state.credits < price {
        // Don't clutter the output with a warning if the upgrade is made from the Auto-Scaling
        if !auto_scale {
            println!("Not enough credits!");
        }
        return Ok(());
    }

    game_state.credits -= price;
    game_state.credits = (game_state.credits * 100.0).round() / 100.0;

    match resource_id {
        CPU_ID => game_state.cpu += 1,
        RAM_ID => game_state.ram += 1,
        DISK_ID => game_state.disk += 1,
        _ => return Ok(()),
    }

    statefile::save(game_state).context("Failed to save game")?;

    // Don't clutter the output with a success message if the upgrade is made from the Auto-Scaling
    if !auto_scale {
        println!("{resource_name} upgraded successfully!");
    }

    Ok(())
}

// Helper to buy perk
fn buy_perk(
    game_state: &mut GameState,
    perk_id: &str,
    perk_name: &str,
    price: f64,
) -> anyhow::Result<()> {
    if game_state.credits < price {
        println!("Not enough credits!");
        return Ok(());
    }

    match perk_id {
        CI_CD_ID if !game_state.ci_cd_unlocked => {
            game_state.ci_cd_unlocked = true;
            game_state.ci_cd_enabled = true
        }

        AUTO_SCALING_ID if !game_state.auto_scaling_unlocked => {
            game_state.auto_scaling_unlocked = true;
            game_state.auto_scaling_enabled = true
        }
        _ => {
            println!("{perk_name} already unlocked!");
            return Ok(());
        }
    }

    game_state.credits -= price;
    game_state.credits = (game_state.credits * 100.0).round() / 100.0;

    statefile::save(game_state).context("Failed to save game")?;

    println!("{perk_name} unlocked successfully!");

    Ok(())
}

// Helper to automatically buy resource upgrades via the Auto-Scaling
pub fn auto_scale(game_state: &mut GameState) -> anyhow::Result<()> {
    // Don't do anything if Auto-Scaling is not enabled
    if !game_state.auto_scaling_enabled {
        return Ok(());
    }

    // Track number of upgrades
    let mut upgrades_count = 0;

    println!("Run Auto-Scaling...");

    // Loop around the various resources, check and sort their upgrade prices in ascending order and
    // automatically upgrade what we can
    loop {
        let cpu_price = economy::cpu_price(game_state.cpu + 1);
        let ram_price = economy::ram_price(game_state.ram + 1);
        let disk_price = economy::disk_price(game_state.disk + 1);

        let cheapest_upgrade = [
            (CPU_ID, "CPU", cpu_price),
            (RAM_ID, "RAM", ram_price),
            (DISK_ID, "Disk", disk_price),
        ]
        .into_iter()
        .filter(|(_, _, price)| game_state.credits >= *price)
        .min_by(|a, b| a.2.total_cmp(&b.2));

        let Some((resource_id, resource_name, price)) = cheapest_upgrade else {
            break;
        };

        upgrade_resource(game_state, resource_id, resource_name, price, true)?;
        upgrades_count += 1;

        println!("{resource_name} automatically upgraded!");
    }

    // Show a message if no upgrades were made
    if upgrades_count == 0 {
        println!("No upgrades affordable.");
    }

    println!();
    Ok(())
}
