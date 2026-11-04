//! Show game menu

use std::io::{self, Write};
use std::{env, process};

pub fn show_menu() -> String {
    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    print!("[{username}@incrust ~]$ ");

    io::stdout().flush().unwrap_or_else(|error| {
        eprintln!("Error: Can't flush stdout\n{error}");
        process::exit(3);
    });

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap_or_else(|error| {
        eprintln!("Error: Can't read stdin\n{error}");
        process::exit(3);
    });

    command
}
