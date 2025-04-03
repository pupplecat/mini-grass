#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status
set -o pipefail  # Pipe failure will be considered as command failure

# delete ledger directory
rm -rf ./test-ledger

# Start solana-test-validator with anchor programs
solana-test-validator --reset --ledger ./test-ledger \
  --bpf-program qL45T6VenxJA8RNaNupKW2K5am8KF1jWTJHEmHqzhGf ./target/deploy/bw_recorder.so
