# Mini-Grass: Bandwidth Reporting to Solana

`Mini-Grass` is a proof-of-concept demonstrating a decentralized bandwidth reporting system. It simulates nodes reporting usage via a REST API, batches contributions, and records them to a Solana smart contract. Built with Rust, this project showcases clean, modular code for backend and blockchain integration.

## Project Structure

This is a mono-repo with three packages:

- **`coordinator`** (`./services/coordinator/`): REST API to record bandwidth from nodes.
- **`rollup`** (`./services/rollup/`): Service to periodically commit bandwidth data to Solana.
- **`bw-recorder`** (`./programs/bw-recorder/`): Solana program to store node contributions on-chain.

## Prerequisites

- **Rust**: `cargo` (1.75+ recommended) - [Install](https://www.rust-lang.org/tools/install)
- **Solana CLI**: `solana` (1.18+) - [Install](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor CLI**: `v0.30.1` - [Install](https://www.anchor-lang.com/docs/installation)
- **Docker**: For containerization - [Install](https://docs.docker.com/get-docker/)

## Getting Started

### 1. Build the Solana Program

```bash
anchor build
```

### 2. Generate a Payer Keypair

```bash
solana-keygen new -o ./payer.json
```

### 3. Configure Environment

Create `.env` in the root directory:

```env

# Coordinator API
HOST=0.0.0.0
PORT=8080

# Rollup schedule (every 5 minutes, cron format)
JOB_SCHEDULE="0 0/5 * * * *"

# Local storage for bandwidth data
BW_FILENAME=./bw_file.json

# Solana config
RPC_URL=http://localhost:8899
PAYER_KEYPAIR_FILENAME=./payer.json
```

### 4. Deploy the Solana Program

Start local test validator.

Optionally, the program has already been deployed to `Devnet`. The program ID is `Pwr6Zo12iYxEqqeaLsWXcaCuw5bw5M3QdZxFnULhjmU` (change the `RPC_URL` in `.env` file to `https://api.devnet.solana.com`).

```bash
# start local test validator
solana-test-validator --reset --ledger ./test-ledger \
  --bpf-program Pwr6Zo12iYxEqqeaLsWXcaCuw5bw5M3QdZxFnULhjmU ./target/deploy/bw_recorder.so

```

### 5. Run the Coordinator Service

```bash
cargo run --package coordinator --bin main
```

### 6. Run the Rollup Service

In separate terminal

```bash
cargo run --package rollup --bin main
```

### 7. Test Bandwidth Reporting

In another terminal, send a report:

```bash

curl -X POST http://localhost:8080/api/report/bandwidth \
  -H "Content-Type: application/json" \
  -d '{"node_id": 7, "bandwidth": 10}'

```

Expected response:

```json
{
    "node_id": 7,
    "status": "recorded",
    "pending_sync": 10
}
```

### 8. Verify On-Chain

Check the program’s data on `Devnet` using a [Solana explorer](https://solscan.io/account/Pwr6Zo12iYxEqqeaLsWXcaCuw5bw5M3QdZxFnULhjmU?cluster=devnet)

For local network use [this link](https://solscan.io/account/Pwr6Zo12iYxEqqeaLsWXcaCuw5bw5M3QdZxFnULhjmU?cluster=custom&customUrl=http://localhost:8899)
