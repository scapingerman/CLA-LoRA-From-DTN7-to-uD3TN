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
     cd ../..
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
  Edit `config.json` for the LoRa CLA. Moroe information in [config/readme_cfg.md](config/readme_cfg.md)


### 2. Flash LoRa Modules

- Open `src/lora/LoRa_TX.ino` in the Arduino IDE and upload it to the transmitter module and `src/lora/LoRa_RX.ino` for the receiver module,

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

- **3. Verify Communication:**
- Check the logs of each component to verify data transmission.
- Confirm that the message "Hello, LORA!" is received by the destination.

### Additional Notes








