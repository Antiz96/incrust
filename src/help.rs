//! Print help message

pub fn show_help() {
    println!("Incrust - A minimal incremental / idle terminal game about compiling stuff");
    println!();
    println!("Run `incrust` to start the game.");
    println!(
        "Then type 'help' to see the list of available commands and 'guide' to see the game guide."
    );
    println!();
    println!("Options:");
    println!("  -h, --help     Display this message");
    println!("  -V, --version  Display version information");
}
