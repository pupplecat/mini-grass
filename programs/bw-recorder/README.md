# BW Recorder Program

The BW Recorder Program is a Solana program designed to record and track contributor bandwidth usages.

## Instructions

- **Initialize:**
  Initializes the state account.

- **RecordBandwidth:**
  Records contributions and updates the overall statistics.

## Accounts

- **Recorder:**
  A PDA account with the seed `['state']` that stores the overall statistics.

- **Contributor:**
  A PDA account with the seeds `['contributor', node_id]` that stores the individual node's contribution.
