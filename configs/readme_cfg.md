### `config.json` Description

This file configures the integration between DTN7, LoRa, and WebSocket for handling and transmitting bundles.

#### Key Sections

- **`lora`**: Defines the LoRa agent.
  - `agent_type`: Specifies the communication type (`websocket`).
  - `agent_arg`: WebSocket address (`0.0.0.0:8291`) and node ID (`node1`).

- **`ecla`**: Configures the External Convergence Layer Adapter (ECLA).
  - `addr`: Address for ECLA to communicate with DTN7.
  - `module_name`: Name of the ECLA module (`LoRa`).

- **`dtnd_executable`**: Path to the DTN7 daemon (`dtnd`).

- **`dtnd`**: Arguments for initializing `dtnd`.
  - `args`: Configures WebSocket port, routing strategy, node ID (`node1`), and ECLA integration.

- **`strategy`**: Routing strategy configuration.
  - `name`: Specifies `quadrant` routing.
  - `SEND_INTERVAL`: Time interval for sending bundles (22 seconds).

- **`airtime`**: LoRa transmission parameters.
  - Configures preamble, spreading factor, bandwidth, coding rate, and CRC.

- **`web_port`**: REST API port (7262).

#### WebSocket Integration
- The WebSocket at `0.0.0.0:8291` handles bundles received from DTN7 and transmits them via LoRa.
- It ensures proper formatting (e.g., CBOR) before interacting with uD3TN.
