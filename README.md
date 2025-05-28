# Metaplex Bubblegum Whitelabel

A whitelabel solution for integrating Metaplex Bubblegum compressed NFTs version 2 into your Solana program via CPI calls.

## Overview

This repository provides a template and toolkit for programs that need to interact with Metaplex Bubblegum's compressed NFT infrastructure on Solana. It leverages Anchor 0.31.1 to provide type-safe CPI (Cross-Program Invocation) interfaces.

## Features

- Pre-configured Anchor 0.31.1 setup for Bubblegum integration
- Type-safe CPI interfaces for common Bubblegum operations
- Examples for minting, transferring, and burning compressed NFTs
- Utility functions for working with Merkle trees
- Test suite demonstrating core functionality

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Anchor 0.31.1
- Node.js and npm/yarn (for tests and examples)

## Installation

```bash
git clone https://github.com/yourusername/play-mpl-bubblegum.git
cd play-mpl-bubblegum
yarn install
```

## Usage

- create_tree: Create a new Merkle tree for compressed NFTs.
- mint_nft: Mint a new compressed NFT into the tree.
- create_collection: Create a new collection for compressed NFTs.
