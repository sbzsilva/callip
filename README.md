# callip

[![Rust Version](https://img.shields.io/badge/rust-1.60+-blue.svg)](https://rust-lang.github.io/rustup/installation/)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/sbzsilva/callip?style=flat)
![GitHub last commit](https://img.shields.io/github/last-commit/sbzsilva/callip?style=flat)
![ViewCount](https://views.whatilearened.today/views/github/sbzsilva/callip.svg?cache=remove)

A simple Rust command-line utility to retrieve and display your public IP address.

## Features

- Retrieves your public IP address using an external API
- Simple and fast command-line interface
- Cross-platform support (Windows, macOS, Linux)
- Help documentation and usage examples

## Installation

### From Source

1. Install Rust and Cargo:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```sh
   git clone https://github.com/sbzsilva/callip.git
   cd callip
   ```

3. Build the application:
   ```sh
   cargo build --release
   ```

4. Install the binary:
   ```sh
   sudo cp target/release/callip /usr/local/bin/
   ```

### Using Cargo

```sh
cargo install --git https://github.com/sbzsilva/callip.git
```

## Usage

### Basic Usage

```sh
callip
```

This will display your public IP address:
```
Your public IP address is: [0.0.0.0]
```

### Help Information

```sh
callip --help
```

This will display help information:
```
Gets the public IP address of the host

Usage: callip

Options:
  -h, --help     Prints help information
  -h, --help     Print help
  -V, --version  Print version

For more information, visit https://github.com/sbzsilva/callip
```

## Development

### Setting Up the Development Environment

1. Install Rust and Cargo:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```sh
   git clone https://github.com/sbzsilva/callip.git
   cd callip
   ```

3. Install dependencies:
   ```sh
   cargo build
   ```

### Running the Application

```sh
cargo run
```

### Building for Release

```sh
cargo build --release
```
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
