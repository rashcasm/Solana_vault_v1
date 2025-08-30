# Blueshift Anchor Vault

Blueshift Anchor Vault is a Solana program built using the Anchor framework. It provides vault functionality for managing assets on the Solana blockchain.

## Features
- Solana smart contract (program) for asset vault management
- Anchor framework for streamlined Solana development

## Project Structure
```
Anchor.toml           # Anchor configuration
Cargo.toml            # Rust dependencies for the main project
package.json          # Node.js dependencies for TypeScript scripts
app/                  # (Optional) Frontend or client app
migrations/           # Deployment scripts
programs/
  blueshift_anchor_vault/
    src/lib.rs        # Main Solana program logic
    Cargo.toml        # Rust dependencies for the program
    Xargo.toml        # Xargo configuration (if used)
target/               # Build artifacts
idl/                  # Anchor-generated IDL files
release/              # Release build artifacts
types/                # TypeScript bindings
  blueshift_anchor_vault.ts
tests/                # TypeScript tests
  blueshift_anchor_vault.ts
```

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://book.anchor-lang.com/chapter_1/installation.html)
- [Node.js](https://nodejs.org/)

### Build and Deploy
1. **Build the program:**
   ```zsh
   anchor build
   ```
2. **Deploy to localnet or devnet:**
   ```zsh
   anchor deploy
   ```

### Testing
- **Rust tests:**
  ```zsh
  cargo test --manifest-path programs/blueshift_anchor_vault/Cargo.toml
  ```
- **TypeScript tests:**
  ```zsh
  anchor test
  ```