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

Run `incrust` to start the game.  
Then type 'help' to see the list of available commands and 'guide' to see the game guide.

## Installation

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
Then type 'help' to see the list of available commands.

## Documentation

See also `incrust --help` and the [incrust(1) man page](https://raw.githubusercontent.com/Antiz96/incrust/refs/heads/main/doc/man/incrust.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/incrust/blob/main/CONTRIBUTING.md).

## License

Incrust is licensed under the [GPL-3.0 license](https://github.com/Antiz96/incrust/blob/main/LICENSE) (or any later version of that license).
