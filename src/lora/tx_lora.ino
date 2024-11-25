#include "LoRaWan_APP.h"
#include "Arduino.h"

// LoRa Configuration
#define RF_FREQUENCY                                868100000 // Hz
#define TX_OUTPUT_POWER                             5         // dBm
#define LORA_BANDWIDTH                              0         // [0: 125 kHz, 1: 250 kHz, 2: 500 kHz, 3: Reserved]
#define LORA_SPREADING_FACTOR                       7         // [SF7..SF12]
#define LORA_CODINGRATE                             1         // [1: 4/5, 2: 4/6, 3: 4/7, 4: 4/8]
#define LORA_PREAMBLE_LENGTH                        8         // Same for Tx and Rx
#define LORA_SYMBOL_TIMEOUT                         0         // Symbols
#define LORA_FIX_LENGTH_PAYLOAD_ON                  false
#define LORA_IQ_INVERSION_ON                        false
#define RX_TIMEOUT_VALUE                            1000
#define BUFFER_SIZE                                 1024      // Define the payload size here

char txpacket[BUFFER_SIZE];   // Buffer for the LoRa message
bool lora_idle = true;        // Status to check if LoRa is idle
static RadioEvents_t RadioEvents;

// Function prototypes for LoRa callbacks
void OnTxDone(void);
void OnTxTimeout(void);

void setup() {
    // Initialize serial communication
    Serial.begin(115200);
    Mcu.begin(HELTEC_BOARD, SLOW_CLK_TPYE);

    // Initialize LoRa radio events
    RadioEvents.TxDone = OnTxDone;
    RadioEvents.TxTimeout = OnTxTimeout;

    // Set up LoRa radio configuration
    Radio.Init(&RadioEvents);
    Radio.SetChannel(RF_FREQUENCY);
    Radio.SetTxConfig(MODEM_LORA, TX_OUTPUT_POWER, 0, LORA_BANDWIDTH,
                      LORA_SPREADING_FACTOR, LORA_CODINGRATE,
                      LORA_PREAMBLE_LENGTH, LORA_FIX_LENGTH_PAYLOAD_ON,
                      true, 0, 0, LORA_IQ_INVERSION_ON, 3000);
}

void loop() {
    // Check if LoRa is idle and if there is data available on the serial port
    if (lora_idle && Serial.available() > 0) {
        // Read the incoming message as raw bytes
        int len = Serial.readBytes(txpacket, BUFFER_SIZE);

        // Truncate the message if it exceeds the buffer size
        if (len > BUFFER_SIZE) {
            Serial.println("Message too large, truncating...");
            len = BUFFER_SIZE;
        }

        // Print the raw content (as binary bytes)
        Serial.print("Sending packet: ");
        for (int i = 0; i < len; i++) {
            Serial.write(txpacket[i]);  // Print each byte as is
        }
        Serial.printf(", length: %d\r\n", len);

        // Send the LoRa packet
        Radio.Send((uint8_t *)txpacket, len);  // Send the binary packet
        lora_idle = false;
    }
    Radio.IrqProcess();  // Process LoRa interrupts
}

// Callback when transmission is completed
void OnTxDone(void) {
    Serial.println("Transmission completed...");
    lora_idle = true;
}

// Callback when transmission times out
void OnTxTimeout(void) {
    Radio.Sleep();
    Serial.println("Transmission timed out...");
    lora_idle = true;
}
