---
sidebar_position: 3
---

# Getting Started

This guide walks you through setting up the environment to develop and test smart contracts in Sigil on a Unix-like system.

### Install Dependencies

#### MacOS
Install `binaryen` and `brotli` using Homebrew:
```bash
brew install binaryen brotli
```

#### Debian
Install `binaryen` and `brotli`:
```bash
sudo apt install binaryen brotli
```

### Set Up Rust
Sigil is built with Rust, so you need the latest stable Rust version installed.

1. Install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) or update to the latest stable version.
2. Add the WebAssembly target for compiling Sigil contracts:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Working with the Sigil ZIP Package

You will receive a ZIP package containing two Rust workspaces. Follow these steps to get started:

1. **Unzip the Package**: Extract the ZIP file provided to you via messaging (e.g., Telegram). The extracted directory has the following structure:
   ```
   sigil-package/
   ├── Kontor/
   │   ├── src/
   │   ├── Cargo.toml
   │   └── ...
   └── sigil-example-contracts/
       ├── hello-world/
       ├── token/
       └── ...
   ```
   - `kontor/`: Contains the core implementation and dependencies for Sigil. You don't need to modify this directly.
   - `sigil-example-contracts/`: Contains a set of example contracts that highlight some of Sigil's most important features.

2. **Navigate to the Example Contract**:
   ```bash
   cd sigil-example-contracts/hello-world
   ```

3. **Run the Tests**:
   Inside the `hello-world` directory, run the following to execute the tests for the example contract:
   ```bash
   cargo test
   ```

## Next Steps
The `hello-world/` example contracts provides a basic Sigil smart contract template. You can:
- Work directly in the `hello-world/` directory to experiment with the example contract. Avoid opening the entire `sigil-example-contracts/` folder in your editor as the multiple contract workspaces contained within will confuse the Rust language server.
- Copy and paste the `hello-world/` folder to create new contracts, using it as a starting point.