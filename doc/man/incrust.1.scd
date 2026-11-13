incrust(1) ["Incrust 1.1.1" ["Incrust Manual"]]

# Name

Incrust - A minimal incremental / idle terminal game about compiling stuff

# DESCRIPTION

Start compiling to earn credits, invest them in system upgrades and perks, compile faster, earn more, and repeat!

Run `incrust` to start the game.

Then type `help` to see the list of available commands and `guide` to see the game guide.

# SYNOPSIS

*incrust* [OPTION]

# OPTIONS

*-V, --version*
	Display version information.

*-h, --help*
	Display the help message.

# EXIT STATUS

*0* Ok.

*1* Failed to create or acquire the lockfile.

*2* Failed to create, read or write the statefile.

*3* Failed to initialize the command prompt editor.

*4* Failed to register the interruption shortcut (Ctrl+C) handler for compilation.

*5* Failed to retrieve the player's command.

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/incrust

# AUTHOR

Robin Candau <robincandau@mailbox.org>
