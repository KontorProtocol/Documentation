# Kontor Protocol Documentation

Welcome to the official documentation for Kontor, a next-generation metaprotocol on Bitcoin that brings programmable assets and perpetual file storage to the world's most secure blockchain.

## What is Kontor?

Kontor is a Bitcoin metaprotocol that enables:

- **Smart Contracts**: Full-featured smart contracts using WebAssembly and Rust, with native composability and deep Bitcoin integration
- **File Persistence**: Economically-incentivized perpetual data storage with cryptographic proofs anchored to Bitcoin
- **Bitcoin Native**: Every Kontor transaction is a Bitcoin transaction, secured by Bitcoin's proof-of-work consensus

Unlike sidechains or Layer-2s, Kontor operates entirely on Bitcoin itself using embedded consensus. There are no validators, no bridges, no separate consensus mechanism—just deterministic rules applied to Bitcoin's immutable transaction history.

## Repository Structure

This repository contains:

- **`docs/kontor/`** - Kontor Protocol documentation (introduction, architecture, tokenomics, security, FAQ)
- **`docs/sigil/`** - Sigil smart contract framework documentation (getting started, examples, reference)
- **`examples/`** - Full working code for all documented Sigil contract examples
- **`img/`** - Diagrams and images used throughout the documentation

## Getting Started

### For Users

Start with the [Kontor Protocol Introduction](docs/kontor/introduction/introduction.mdx) to learn what Kontor is and why it matters.

### For Developers

1. Learn about [Sigil smart contracts](docs/kontor/smart-contracts/smart-contracts.mdx)
2. Follow the [Getting Started guide](docs/sigil/getting-started/getting-started.mdx)
3. Explore [example contracts](docs/sigil/examples/index/index.mdx)
4. Browse the full source code in the `examples/` directory

## Running the Documentation Site

This documentation site is built using [Mintlify](https://mintlify.com/).

### Installation

```bash
npm install
```

### Local Development

```bash
npx mintlify dev
```

This starts a local development server at `http://localhost:3000` and opens a browser window. Most changes are reflected live without restarting the server.

### Deployment

PRs merged into the `main` branch will trigger a deployment to the production site.

## Contributing

Contributions to the documentation are welcome! Please submit pull requests for any improvements, corrections, or additions.
