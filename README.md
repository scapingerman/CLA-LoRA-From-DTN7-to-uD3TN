# LoRa-DTN Integration

## Overview

This project demonstrates the integration of LoRa communication with Delay-Tolerant Networking (DTN) protocols using DTN7 and uD3TN. 

---

## Repository Structure

```plaintext
.
├── README.md             # Project description and instructions
├── docs/                 # Additional documentation
│   ├── architecture.md   # System architecture details
│   └── setupinstructions.md # Detailed setup steps
├── src/                  # Source code
│   ├── websocket/        # WebSocket integration for data handling
│   │   ├
│   │   ├── TXwebsocket.py
│   │   ├── RXwebsocket_to_ud3tn.py
│   ├── lora/             # LoRa module scripts
│   │   ├── LoRa_TX.ino
│   │   ├── LoRa_RX.ino
│   ├── dtn7-rs-lora-ecla/ # DTN7 with LoRa CLA implementation
│   └── dtn7/             # DTN7 core
└── configs/              # Configuration files
    └── dtn7-rs-lora-ecla-config.json│   

```
# Prerequisites

To run this project, you will need:

- **LoRa Modules**: Compatible LoRa devices such as Heltec ESP32 LoRa or Adafruit Feather M0 LoRa.
- **DTN Software**: DTN7 and uD3TN precompiled or built from source.
- **Python**: For running WebSocket bridges and utility scripts.
- **CMake and GCC**: To compile uD3TN and other native components.
- **Arduino IDE**: For flashing LoRa modules.

---

# Quick Start

## 1. Clone the Repository
```bash
git clone https://github.com/scapingerman/lora-dtn-integration.git
cd lora-dtn-integration
```

## 2. Set Up Components
Follow the instructions in [docs/setupinstructions.md](docs/setupinstructions.md) to configure DTN7, CLA_LoRa, and uD3TN.

## 3. Flash LoRa Modules
- Use `src/lora/LoRa_TX.ino` for the transmitter module.
- Use `src/lora/LoRa_RX.ino` for the receiver.

## 5. Test the System
Transmit sample data using:

```bash
echo "Hello, LORA!" | dtnsend -s dtn://node1/ --receiver dtn://LoRa_Local/echo
```


## Important Notes for Implementing **Heltec LoRa 32 V3** (ESP32-S3) on **Windows 11**

### Important Notes for **Heltec LoRa 32 V3** Users
If you're working with the **Heltec LoRa 32 V3** model, keep in mind that this model uses an **ESP32-S3** microcontroller, which has some key differences compared to the standard ESP32 used in earlier versions of this board. The **ESP32-S3** introduces some additional challenges, especially when working on **Windows 11**.

### Common Issues and Solutions
On **Windows 11**, you may face issues when trying to flash the firmware correctly. One interesting detail is that, in some cases, the operating system requires that the folder path where the project is stored doesn't contain any **spaces** or **special characters**. It might seem trivial, but I spent hours searching for a solution, and the fix turned out to be as simple as renaming the project folder and removing those characters! Although I haven't found the exact link again where I discovered this solution, I hope this helps others in similar situations.

### Using **PlatformIO** with **rf95modem**
For those interested in using **PlatformIO** as firmware for their **Heltec LoRa 32 V3** projects, you can use the **rf95modem** firmware. This is an excellent option for working with LoRa.

- **rf95modem firmware**: [https://github.com/gh0st42/rf95modem](https://github.com/gh0st42/rf95modem)

### Problem Encountered and Solution in the **rf95modem** Repository

I was attempting to implement the **rf95modem** firmware on the **Heltec LoRa 32 V3**. After correctly configuring the pins in the `platformio.ini` file (as shown below), the build was successful, but when I tried to open the serial monitor in **Arduino IDE**, I encountered the following error during LoRa radio initialization:
```bash
ESP-ROM:esp32s3-20210327 Build:Mar 27 2021 rst:0x1 (POWERON),boot:0x8 (SPI_FAST_FLASH_BOOT) SPIWP:0xee mode:DIO, clock div:1 load:0x3fce3808,len:0x4bc load:0x403c9700,len:0xbd8 load:0x403cc700,len:0x2a0c entry 0x403c98d0 rf95modem firmware (v0.7.4) Copyright (c) 2018, 2019 Lars Baumgaertner +FAIL: LoRa radio init
```
This error suggests that the LoRa radio initialization isn't happening correctly. Although I haven't fully resolved this issue, I've raised it in the **rf95modem** repository under the following **issue**:

- **Issue raised in the repository**: [https://github.com/gh0st42/rf95modem/issues/19](https://github.com/gh0st42/rf95modem/issues/19)

Any guidance on how to resolve the LoRa radio initialization issue would be greatly appreciated.

### **platformio.ini** File Configuration
Here is the **platformio.ini** configuration I used to try and get **rf95modem** working with the **Heltec LoRa 32 V3** model:

```ini
[env:esp32s3_heltec_config]
platform = espressif32
board = heltec_wifi_lora_32_V3
framework = arduino
build_flags =
  -fexceptions
  -DRFM95_CS=8
  -DRFM95_RST=12
  -DRFM95_INT=14
  -DUSE_DISPLAY
  -DOLED_ADDRESS=0x3C
  -DOLED_SDA=17
  -DOLED_SCL=18
  -DOLED_RST=21
lib_deps =
  ${env.lib_deps}
  ${env.libs_display}
board_build.partitions = no_ota.csv

This platformio.ini configuration was essential for trying to match the ESP32-S3 pins on the Heltec LoRa 32 V3 with the rf95modem firmware. Although the build was successful, the issue with radio initialization persists and remains under investigation.

