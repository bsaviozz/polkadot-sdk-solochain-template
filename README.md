# Post-Quantum Substrate Solochain

This repository contains a modified version of the Substrate **Solochain Template** used to evaluate **post-quantum signature schemes** (CRYSTALS-Dilithium / ML-DSA) in a blockchain environment.

The runtime integrates Dilithium alongside the standard Substrate signature algorithms:

- `sr25519`
- `ecdsa`
- `ML-DSA (Dilithium)`

The node is used together with a custom client to benchmark:

- key generation latency
- transaction signing latency
- on-chain verification latency
- transaction lifecycle latency
- extrinsic size
- extrinsic weight

---

# Dilithium Runtime Branches

Because different Dilithium parameter sets currently require different versions of the `pq` library, the implementations are separated into different branches.

| Branch                | Dilithium Variant | NIST Name |
| --------------------- | ----------------- | --------- |
| `dilithium-ml-dsa-44` | Dilithium 2       | ML-DSA-44 |
| `dilithium-ml-dsa-65` | Dilithium 3       | ML-DSA-65 |
| `dilithium`           | Dilithium 5       | ML-DSA-87 |

Notes:

- `ML-DSA-44` and `ML-DSA-65` rely on an older version of the `pq` library.
- `ML-DSA-87` uses the newer implementation and mirrors the integration used for `sr25519` and `ecdsa`.

To use a specific Dilithium variant simply change the root Cargo.toml to point to the corresponding runtime branch.

---

# Node Variants

Two versions of the node are provided.

## Standard Node

This version uses the normal Substrate consensus setup:

- **Aura** for block production
- **GRANDPA** for finality

Blocks are produced periodically and finalized after consensus.

Run with:

```bash
./target/release/solochain-template-node --dev
```

## Instant Node

This version enables instant block inclusion.

Each transaction immediately produces a block. This removes the block production delay and allows measuring pure runtime and cryptographic costs without consensus latency.

- Checkout to the "dilithium-instant" branch.
- Build the node
- Run with:

```bash
./target/release/solochain-template-node --dev
```

# Typical workflow:

1. Start the node

   ```bash
   cargo build --release
   ./target/release/solochain-template-node --dev
   ```

   or run the instant node version.

2. Run the client

The client repository provides:

- a benchmark runner that executes all experiments
- a simple transaction client that sends transfers

### Build

🔨 Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Embedded Docs

After you build the project, you can use the following command to explore its
parameters and subcommands:

```sh
./target/release/solochain-template-node -h
```

You can generate and view the [Rust
Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template
with this command:

```sh
cargo +nightly doc --open
```

### Single-Node Development Chain

The following command starts a single-node development chain that doesn't
persist state:

```sh
./target/release/solochain-template-node --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/solochain-template-node purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/solochain-template-node -ldebug --dev
```

Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/src/chain_spec.rs`) that
  includes several pre-funded development accounts.

To persist chain state between runs, specify a base path by running a command
similar to the following:

```sh
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/solochain-template-node --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the
hosted version of the [Polkadot/Substrate
Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
front-end by connecting to the local node endpoint. A hosted version is also
available on [IPFS](https://dotapps.io/). You can
also find the source code and instructions for hosting your own instance in the
[`polkadot-js/apps`](https://github.com/polkadot-js/apps) repository.
