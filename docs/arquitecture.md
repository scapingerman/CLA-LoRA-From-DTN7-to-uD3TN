# System Architecture

## Overview

This project integrates **LoRaWAN** and **DTN** technologies to enable reliable data transmission over intermittently connected networks. The architecture includes custom Convergence Layer Adapters (CLAs) for seamless interoperability between LoRa and DTN protocols.

---

## Workflow Diagram

((( Data Source )))                                                      
       |                                                            
       v                                                            
+-------------------+                                                  
|       DTN7        |                                                  
+-------------------+                                                 
       v                                                                
+-------------------+                                                  
|     CLA_LoRa      |                                                 
+-------------------+                                                  
       |                                                               
       v                                                              
+-------------------+                                                 
|    WebSocket TX   |                                                 
+-------------------+                                                
       |                                                             
       v                                                             
+-------------------+        LoRa medium        +-------------------+ 
|      LoRa_TX      | ---------------------->   |      LoRa_TX      | 
+-------------------+                           +-------------------+
                                                    |                
                                                    v                
                                                +-------------------+
                                                |    WebSocket RX   |
                                                +-------------------+
                                                    |                 
                                                    v                 
                                                +-------------------+
                                                |     CLA_MTCP      |
                                                +-------------------+
                                                    |                 
                                                    v                 
                                                +-------------------+
                                                |      uD3TN        |
                                                +-------------------+





### Components and Responsibilities

1. **Data Source:**  
   Generates the data for transmission.

2. **DTN7 Node:**  
   Bundles the data using the DTN protocol.

3. **CLA_LoRa (Convergence Layer Adapter for LoRa):**  
   Encapsulates DTN bundles for LoRa transmission.

4. **WebSocket Bridge:**  
   Manages communication between CLAs and LoRa modules.

5. **LoRa_TX and LoRa_RX Modules:**  
   Handle the physical transmission and reception of data over LoRa.

6. **CLA_MTCP (Convergence Layer Adapter for MTCP):**  
   Re-adapts received data for the **uD3TN** DTN stack.

7. **uD3TN Node:**  
   Processes DTN bundles for end applications.

---

## Physical Architecture

- **LoRa_TX and LoRa_RX** communicate over a long-range, low-power wireless channel.
- **DTN7** and **uD3TN** are implemented on separate nodes or devices.
- WebSocket interfaces act as intermediate communication layers.

## Key Features

- Seamless integration of DTN and LoRa protocols.
- Customizable configuration for different scenarios.
