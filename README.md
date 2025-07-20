# aur-dependency-tracker

A CLI tool for tracking remote and local AUR (Arch User Repository) package dependencies. The core dependency logic is implemented in Rust for performance and reliability, with a Bash wrapper for easy usage.

## Features
- Query AUR for package dependencies (`remote`)
- Inspect local pacman database for installed package dependencies (`local`)
- (Planned) Track and list packages persistently (`list`)

## Structure
- `bash/cli.sh` — Bash CLI entrypoint
- `rust/` — Rust core logic (to be compiled as a binary)
- `build.sh` — Build script to compile Rust and wire up with Bash

## Usage

### Build

```sh
./build.sh
```

### Run

```sh
bash/cli.sh <SUBCOMMAND> [ARGS]
```

#### Examples

- Query remote AUR package dependencies:
  ```sh
  bash/cli.sh remote yay
  ```
- Query local package dependencies (installed):
  ```sh
  bash/cli.sh local yay
  ```
- List tracked packages (placeholder):
  ```sh
  bash/cli.sh list
  ```

## Installation

1. Clone the repository:
   ```sh
   git clone <repo-url>
   cd aur-dependency-tracker
   ```
2. Build the project:
   ```sh
   ./build.sh
   ```
3. Use the CLI as shown above.

## Development
- Rust code is in `rust/src/main.rs`.
- Bash wrapper is in `bash/cli.sh`.
- Build script is `build.sh`.
- Tests are in the Rust source file (run with `cargo test` in the `rust/` directory).

## Contributing
Pull requests and issues are welcome! Please:
- Follow Rust and Bash best practices.
- Add tests for new features.
- Document your code and update this README as needed.

## License
MIT
