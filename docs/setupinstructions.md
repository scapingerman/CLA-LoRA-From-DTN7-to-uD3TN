# Setup Instructions

## Prerequisites
**Clone and Build Repositories:**
   - Clone the `lora-dtn-integration` repository:
     ```bash
     git clone https://github.com/scapingerman/lora-dtn-integration.git
     cd lora-dtn-integration
     ```
   - Build `dtn7` and `dtn7-rs-lora-ecla`:
     ```bash
     cd src/dtn7-rs-lora-ecla
     cargo build --release
     ```
     More information in https://github.com/BigJk/dtn7-rs-lora-ecla and https://github.com/dtn7/dtn7-rs
   
   - Clone and build uD3TN:
     ```bash
     git clone https://gitlab.com/d3tn/ud3tn.git
     cd ud3tn
     git submodule update --init --recursive
     make posix     
     ```

      More information in https://gitlab.com/d3tn/ud3tn

## Step-by-Step Setup

### 1. Configure Components

- **CLA_LoRa:**
  Edit `config.json` for the LoRa CLA. For more details, see the [configuration readme](../configs/readme_cfg.md).


### 2. Flash LoRa Modules

- Open `src/lora/LoRa_TX.ino` in the Arduino IDE and upload it to the transmitter module and `src/lora/LoRa_RX.ino` for the receiver module.

### 3. Run WebSocket Bridges

Start the WebSocket servers for the LoRa communication bridge using the provided Python scripts:

```bash
sudo python3 src/websocket/TXwebsocket.py
sudo python3 src/websocket/RXwebsocket_to_ud3tn.py
```

### 4. Run WebSocket Bridges
- **Run the DTN7 service:**
```bash
./src/dtn7-rs-lora-ecla/target/release/loclad -c configs/dtn7-rs-lora-ecla-config.json
```

- **Start the uD3TN nodes:**
```bash
build/posix/ud3tn --eid dtn://LoRa_Local/ --bp-version 7 --aap2-port 4242 --cla "mtcp:*,4224" -L 4
build/posix/ud3tn --eid dtn://node1/ --bp-version 7 --aap2-port 4243 --cla "mtcp:*,4225" -L 4
```

### 5. Run WebSocket Bridges
- **Use the provided AAP2 configuration script to establish a connection:**
```bash
python tools/aap2/aap2_config.py --tcp localhost 4242 --schedule 1 3600 100000 dtn://node1/ mtcp:localhost:4225
```

### 6. Test the Workflow

- **1. Start all services:**
- Ensure DTN7, CLA_LoRa, LoRa_TX, and LoRa_RX modules are operational.
- Confirm that WebSocket servers are running.

- **2. Send a test message:**
- Transmit data from DTN7 through LoRa to uD3TN:
```bash
echo "Hello, LORA!" | dtnsend -s dtn://node1/ --receiver dtn://LoRa_Local/echo
```
- Command Breakdown:
   - Creates a bundle: The payload "Hello, LORA!" is wrapped as a DTN bundle.
   - Defines the sender: dtn://node1/ acts as the sending node.
   - Defines the receiver: dtn://LoRa_Local/echo is the target node.


The payload is encapsulated into a bundle transmitted from node1 to LoRa_Local. The WebSocket connected to the LoRa CLA captures the message in bytes:
   ```bash
               b'\x9f\x88\x07\x1a\x00\x02\x00\x04\x00\x82\x01q//LoRa_Local/echo\x82\x01h//node1/\x82\x01h//node1/\x82\x1b\x00\x00\x00\xb6\xdaD\xf6\x94\x00\x1a\x006\xee\x80\x85\x06\x03\x00\x00K\x82\x01h//node1/\x85\n\x02\x00\x00D\x82\x18 \x01\x85\x01\x01\x00\x00MHello, LORA!\n\xff'
   ```

Decoded into CBOR, the bundle structure appears as follows:
   ```bash
   [
     [7, 131076, 0, [1, "//LoRa_Local/echo"], [1, "//node1/"], [1, "//node1/"], [785346000532, 0], 3600000],
     [6, 3, 0, 0, "b'\\x82\\x01h//node1/'"],
     [10, 2, 0, 0, "b'\\x82\\x18 \\x01'"],
     [1, 1, 0, 0, "b'Hello, LORA!\\n'"]
   ]
   ```
The bundle contains the essential information for routing and transmitting data across a DTN network. It includes the source, destination, and report-to endpoints, a timestamp, a lifetime for the bundle, and the payload message ('Hello, LORA!'). Each component is structured for efficient data exchange in a delay-tolerant network.


- **3. Forwarding to uD3TN**
The bundle is transmitted from the LoRa end device to another node. It is processed by a WebSocket, which prepares the bundle for delivery to uD3TN through the cla_mtcp CLA.

