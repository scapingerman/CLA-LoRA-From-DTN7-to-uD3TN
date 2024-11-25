#include <WiFi.h>
#include <ArduinoWebsockets.h>
#include "LoRaWan_APP.h"
#include "Arduino.h"

// Wi-Fi credentials
const char* ssid = "D3TN"; 
const char* password = "eJoDM{_l<cMEt@@EPR_@;{2*J"; 

// WebSocket server configuration
const char* websockets_server_host = "192.168.8.95"; 
const uint16_t websockets_server_port = 8292; 

using namespace websockets;

WebsocketsClient client;

// LoRa parameters
#define RF_FREQUENCY 868100000 // Hz
#define TX_OUTPUT_POWER 5      // dBm
#define LORA_BANDWIDTH 0       // [0: 125 kHz, 1: 250 kHz, 2: 500 kHz, 3: Reserved]
#define LORA_SPREADING_FACTOR 7 // [SF7..SF12]
#define LORA_CODINGRATE 1       // [1: 4/5, 2: 4/6, 3: 4/7, 4: 4/8]
#define LORA_PREAMBLE_LENGTH 8  // Same for Tx and Rx
#define LORA_SYMBOL_TIMEOUT 0   // Symbols
#define LORA_FIX_LENGTH_PAYLOAD_ON false
#define LORA_IQ_INVERSION_ON false

#define RX_TIMEOUT_VALUE 1000
#define BUFFER_SIZE 1024 // Payload size

char rxpacket[BUFFER_SIZE];

static RadioEvents_t RadioEvents;
bool lora_idle = true;

// Reconnect WebSocket if disconnected
void reconnectWebSocket() {
    if (!client.available()) {
        Serial.println("Attempting to reconnect WebSocket...");
        bool connected = client.connect(websockets_server_host, websockets_server_port, "/");
        if (connected) {
            Serial.println("Reconnected to WebSocket server");
        } else {
            Serial.println("Error reconnecting WebSocket");
        }
    }
}

void setup() {
    Serial.begin(115200);
    Mcu.begin(HELTEC_BOARD, SLOW_CLK_TPYE);

    // Connect to Wi-Fi
    WiFi.begin(ssid, password);
    while (WiFi.status() != WL_CONNECTED) {
        delay(1000);
        Serial.print(".");
    }
    Serial.println("\nWi-Fi connected");
    Serial.println("IP Address: ");
    Serial.println(WiFi.localIP());

    // Connect to WebSocket server
    bool connected = client.connect(websockets_server_host, websockets_server_port, "/");
    if (connected) {
        Serial.println("Connected to WebSocket server");
    } else {
        Serial.println("Could not connect to WebSocket server");
    }

    client.onMessage([](WebsocketsMessage message) {
        Serial.print("Message received: ");
        Serial.println(message.data());
    });

    // Configure LoRa
    RadioEvents.RxDone = OnRxDone;
    Radio.Init(&RadioEvents);
    Radio.SetChannel(RF_FREQUENCY);
    Radio.SetRxConfig(MODEM_LORA, LORA_BANDWIDTH, LORA_SPREADING_FACTOR,
                      LORA_CODINGRATE, 0, LORA_PREAMBLE_LENGTH,
                      LORA_SYMBOL_TIMEOUT, LORA_FIX_LENGTH_PAYLOAD_ON,
                      0, true, 0, 0, LORA_IQ_INVERSION_ON, true);
}

void loop() {
    if (lora_idle) {
        lora_idle = false;
        Serial.println("RX mode activated");
        Radio.Rx(0); // Start receiving
    }
    
    Radio.IrqProcess();      // Process LoRa interrupts
    client.poll();           // Keep WebSocket connection alive
    reconnectWebSocket();    // Retry connection if down
}

void OnRxDone(uint8_t* payload, uint16_t size, int16_t rssi, int8_t snr) {
    // Copy received payload
    memcpy(rxpacket, payload, size);
    rxpacket[size] = '\0';  // Ensure the received packet is a valid string
    Radio.Sleep();          // Put LoRa in sleep mode after receiving
    Serial.printf("\nPacket received: \"%s\" with RSSI: %d, Length: %d\n", rxpacket, rssi, size);
    lora_idle = true;       // Return to RX mode in the next iteration

    if (client.available()) {
        // Send data as binary to WebSocket
        client.sendBinary(reinterpret_cast<const char*>(payload), size);  
        Serial.println("Message sent to WebSocket in binary format");
    } else {
        Serial.println("WebSocket not available to send data");
    }
}