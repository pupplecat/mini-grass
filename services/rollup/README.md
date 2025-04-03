# RollUp Service

The RollUp Service is organized using clean architecture principles. It is structured into the following layers:

1. **Transport:**
   Handles event triggers via a job scheduler.

2. **Usecase:**
   Manages business logic, processes scheduled data, and coordinates with the repository.

3. **Repository:**
   Manages connectivity to external systems, including persistence and on-chain interactions.

---

## Environment Variables

Configure the service using the following environment variables:

  ```env
  # Path to the target storage file
  BW_FILENAME=./bw_file.json

  # Job schedule (cron expression) for syncing information hourly
  JOB_SCHEDULE="0 0 * * * *"

  # Solana configuration
  RPC_URL=https://api.devnet.solana.com
  PAYER_KEYPAIR_FILENAME=./payer.json
```

## Example Usage

The service read information from `BW_FILENAME` file, for each scheduled interval it will build record_bandwidth instructions and submit to Solana program

  ```json
  {
    "123": 1000,
    "124": 500
  }
  ```

## Additional Notes

- **Architecture:** The service follows clean architecture patterns, promoting separation of concerns and easier maintenance.
- **Extensibility:** Each layer (Transport, Usecase, Repository) can be extended or modified independently to meet evolving requirements.
- **Configuration:** Adjust the environment variables as necessary to meet your deployment needs.
