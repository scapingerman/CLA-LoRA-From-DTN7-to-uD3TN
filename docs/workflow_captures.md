# DTN7 LoRa-DTN7 CLA Integration

Below are the screenshots illustrating the workflow of the DTN7 LoRa-DTN7 CLA integration.

## Screenshots

![Data Source](Data_Source.png)
**Screenshot 1** - Data source: Generates data to be transmitted.

![DTN7 Node](DTN7_Node.png)
**Screenshot 2** - DTN7 Node: Bundles data using DTN protocols.

![CLA LoRa](CLA_LoRa.png)
**Screenshot 3** - CLA LoRa: Encapsulates DTN bundles for LoRa transmission.

### Transmission and Reception

![Bundle Transmission](Bundle_Transmission.png)
**Screenshot 4** - Serial print for the bundle transmission in LoRa TX.

![Bundle Reception](Bundle_Reception.png)
**Screenshot 5** - Bundle reception in WebSocket RX.

![CLA MTCP Received Data](CLA_MTCP-Received_data.png)
**Screenshot 6** - CLA MTCP: Adapts received data for µD3TN nodes.

![Processed Bundle](Processed_Bundle.png)
**Screenshot 7** - µD3TN Node: Processes bundles for end applications.
