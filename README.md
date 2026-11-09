# Incrust

## Table of contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Description

A minimal incremental / idle terminal game about compiling stuff.  
Start compiling to earn credits, invest them in system upgrades and perks, compile faster, earn more, and repeat!

Run `incrust` to start the game.  
Then type `help` to see the list of available commands and `guide` to see the game guide.

See the quick video demo below:

<https://github.com/user-attachments/assets/edf79f49-9c9f-43ab-a2df-bf13e1d7934f>

## Installation

## Runtime dependencies

- [`genact`](https://github.com/svenstaro/genact) (optional) - For compilation animations

### Packages

[![Packaging status](https://repology.org/badge/vertical-allrepos/incrust.svg)](https://repology.org/project/incrust/versions)

### Pre-compiled binary

A (statically linked) pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a [release asset](https://github.com/Antiz96/incrust/releases/latest) (`incrust-<release_version>-x86_64`).

The pre-compiled binary can be reproduced from source (in the sense of [reproducible builds](https://reproducible-builds.org)).  
The build environment is created and fully documented via [repro-env](https://github.com/kpcyrd/repro-env), and is tracked in this repository.

To reproduce the pre-compiled binary for a given release, [install repro-env](https://github.com/kpcyrd/repro-env#download) and run the following:

```bash
git clone https://github.com/Antiz96/incrust.git
cd incrust
git checkout <tag> # Where <tag> is the git tag for the targeted release, e.g. "v1.0.0"
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/incrust
```

Then, compare the `sha256` hash of the built binary to the one of the pre-compiled release binary (which is also recorded in the `incrust-<release_version>-x86_64.sha256` file in the release assets). Both hashes should be equal, indicating that the binary has been successfully reproduced.

Each release assets are also cryptographically signed, with the detached signature for each asset distributed as `<asset_name>.asc` (see the [MAINTAINERS.md file](https://github.com/Antiz96/incrust/blob/main/MAINTAINERS.md) for a list of keys expected to emit signatures).

### Build from source

```bash
git clone https://github.com/Antiz96/incrust.git
cd incrust
cargo build --release
```

The built binary will be located at `./target/release/incrust`, place it somewhere in your `$PATH`.

The [man page](https://github.com/Antiz96/incrust/tree/main/doc/man) can be generated with `scdoc`:

```bash
scdoc < doc/man/incrust.1.scd > doc/man/incrust.1
```

There are also shell completions available in the [`res/completions/`](https://github.com/Antiz96/incrust/tree/main/res/completions) directory.

## Usage

Run `incrust` to start the game.  
Then type 'help' to see the list of available commands and 'guide' to see the game guide (shown below).

```text
Start compiling stuff by running the 'compile' command.
You'll earn credits for each completed (and successful) compilation.
Compilations have a small chance to fail upon completion, in which case
you won't get rewarded with credits for it.
Ongoing compilation can be interrupted by pressing Ctrl+C, but note that
you won't get any credit for it either.

Spend credits to upgrade your system resources and buy different perks by running
the 'upgrade' command:

- Buying CPU cores speeds up compile time
  but also slightly increases chances of compilation failures
- Buying RAM increases the credit reward per completed (and successful) compilation
  but also slightly increases the compile time
- Buying Disk space reduces chances of compilation failures
  but also slightly reduces the credit reward per completed (and successful) compilation
- Other bonuses can be acquired through different one-time purchase perks

Unlocked perks can be enabled or disabled by running the 'manage' command.

You can check the current status of your resources and perks by running the 'status' command.

Progression is automatically saved and resumed between game sessions.
```

## Documentation

See also `incrust --help` and the [incrust(1) man page](https://raw.githubusercontent.com/Antiz96/incrust/refs/heads/main/doc/man/incrust.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/incrust/blob/main/CONTRIBUTING.md).

## License

Incrust is licensed under the [GPL-3.0 license](https://github.com/Antiz96/incrust/blob/main/LICENSE) (or any later version of that license).
