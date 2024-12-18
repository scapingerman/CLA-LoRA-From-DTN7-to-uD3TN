import asyncio
import websockets
import serial

# Serial port configuration for LoRa
ser = serial.Serial('/dev/ttyUSB0', 115200)

# Check if the message contains the desired byte (\x9f) (the beginning of the bundle)
def is_desired_message(message):
    if b'\x9f' in message:
        return True
    return False

# Filter and extract the message starting from \x9f
def extract_from_9f(message):
    # Look for the byte \x9f in the message
    start_index = message.find(b'\x9f')
    if start_index != -1:
        # Extract only from the first \x9f byte to the end
        return message[start_index:]
    return None

async def handle_message(websocket, path):
    print("Connected Client")
    async for message in websocket:
        print(f"DTN7 message received: {message}")

        # Check if it's a valid binary message
        if isinstance(message, bytes) and is_desired_message(message):
            # Extract from \x9f
            extracted_message = extract_from_9f(message)
            
            if extracted_message:
                # Send the extracted message as binary to the LoRa port
                ser.write(extracted_message)
                print(f"Message sent to LoRa (binary): {extracted_message}")
            else:
                print("Extraction failed. No valid message found.")
        else:
            print("Beacon or invalid message received.")

async def main():
    # Start the WebSocket server
    server = await websockets.serve(handle_message, "0.0.0.0", 8291)
    print("WebSocket server started at ws://0.0.0.0:8291")
    await server.wait_closed()

# Run the main function
asyncio.run(main())

