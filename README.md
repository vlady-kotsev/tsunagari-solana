# Tsunagari Bridge Solana

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Instructions](#instructions)
- [Deployment](#deployment)
- [Testing](#testing)
- [License](#license)

## Overview

**Tsunagari** (to connect) Solana is a cross-chain bridge implementation written in Rust using the Anchor framework. This program enables secure token transfers between Solana and other EVM blockchain networks.

## Features

- Security by requiring a threshold amount of signatures (n of m)
- Support for native and wrapped token transfers
- Configurable fee system
- No reliance on a centralized service
- Signature validation using ECDSA recovery to verify Ethereum signatures

## Instructions

- `Initialize`: Sets up the bridge with initial configuration and members.
- `SetMember`: Adds or removes a validator from the bridge.
- `SetFee`: Updates the fee percentage for cross-chain transfers.
- `SetThreshold`: Changes the required signature threshold for operations.
- `AddSupportedToken`: Registers a new token for cross-chain transfers.
- `RemoveSupportedToken`: Removes a token from the supported list.
- `MintWrapped`: Creates wrapped tokens representing assets from other chains.
- `BurnWrapped`: Burns wrapped tokens when transferring back to origin chain.
- `Lock`: Locks native tokens in the bridge vault for cross-chain transfers.
- `Unlock`: Releases native tokens from the bridge vault to recipients.
- `CreateWrapped`: Creates a new wrapped token mint.

## Deployment

Deployment to Solana devnet:

1. Build

```bash
anchor build
```

```bash
npm run deploy:devnet
```

## Testing

The program is tested with:

### Integration tests using Anchor and Bankrun:

```bash
anchor test
```

### Local validator tests:

```bash
solana-test-validator
anchor test --skip-local-validator
```

### Static code analysis with **Clippy**:

```bash
cargo clippy -- -D warnings
```

## License

This project is licensed under the MIT License:

MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
