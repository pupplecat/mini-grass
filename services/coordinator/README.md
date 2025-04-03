# Coordinator Service

The Coordinator Service is organized using clean architecture principles. It is structured into the following layers:

1. **Transport:**
   Handles incoming requests and outgoing responses (in this case, via a REST API).

2. **Usecase:**
   Manages the business logic associated with incoming requests and interacts with the corresponding repository.

3. **Repository:**
   Manages connectivity to external systems, such as persistence and databases.

---

## Environment Variables

Configure the service using the following environment variables:

  ```env
  # Host on which the service binds
  HOST=0.0.0.0

  # Port on which the service listens
  PORT=8080

  # Path to the target storage file
  BW_FILENAME=./bw_file.json
  ```

## Example Usage

To test the service, you can use a curl command like the one below:

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

## Additional Notes

- **Architecture:** The service follows clean architecture patterns, promoting separation of concerns and easier maintenance.
- **Extensibility:** Each layer (Transport, Usecase, Repository) can be extended or modified independently to meet evolving requirements.
- **Configuration:** Adjust the environment variables as necessary to meet your deployment needs.
