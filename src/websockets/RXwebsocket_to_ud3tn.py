import asyncio
import websockets
import socket
import cbor2

# uD3TN configuration
mtcp_host = "localhost"  # Change to the corresponding IP/host of uD3TN
mtcp_port = 4224         # MTCP port of uD3TN

# Function to handle incoming messages
async def handle_message(websocket, path):
    print("# Client connected to WebSocket\n")
    async for message in websocket:
        print(f"# Message received from LoRa_RX (binary): {message}")

        try:
            # Check if the message needs processing
            print("# Processing received message...")
            # If the message is already in CBOR format, send it directly
            if isinstance(message, bytes):
                try:
                    # Attempt to decode CBOR to ensure it's valid
                    decoded_message = cbor2.loads(message)
                    print(f"# Successfully decoded: {decoded_message}")
                except cbor2.CBORDecodeError:
                    print("# The received message is not in CBOR format")

                # Encapsulate the message in CBOR if not already in that format
                cbor_message = cbor2.dumps(message)

                # Connect and send the message to uD3TN via MTCP
                with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                    s.connect((mtcp_host, mtcp_port))
                    print(f"# Connection established with uD3TN at {mtcp_host}:{mtcp_port}")
                    s.sendall(cbor_message)  # Send the encapsulated message
                    print("# Bundle successfully sent to uD3TN\n")

            else:
                print("# Message is not binary. Discarded.\n")
        except Exception as e:
            print(f"# Error processing the message: {e}")

# Initialize the WebSocket server
async def main():
    server = await websockets.serve(handle_message, "0.0.0.0", 8292)
    print("# WebSocket server started at ws://0.0.0.0:8292")
    await server.wait_closed()

# Run the server
asyncio.run(main())
