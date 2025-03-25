📁 /docs
 ├── 📄 README.md                           # Documentation Overview
 │
 ├── 📁 vision-strategy/                    # 📌 Vision & Strategic Documents
 │   ├── vision.md                            # What is this project? Goals, audience, business case.
 │   ├── architecture-principles.md           # Core guiding principles (modularity, power efficiency, security).
 │   ├── technology-strategy.md               # MCU, RTOS, connectivity, protocols, and stack selection.
 │   ├── roadmap.md                           # Future architecture evolution (e.g., OTA, AI on edge).
 │
 ├── 📁 system-architecture/                 # 📌 High-Level Embedded System Architecture
 │   ├── overview.md                          # High-level system overview (firmware, cloud, hardware interactions).
 │   ├── system-context.md                    # External dependencies (sensors, actuators, cloud services, gateways).
 │   ├── solution-strategy.md                 # Trade-offs (real-time vs low-power, cloud vs edge processing).
 │   ├── embedded-software-structure.md       # Firmware architecture (RTOS, HAL, middleware, bootloader).
 │   ├── hardware-abstraction.md              # MCU, peripherals, power management, board layout.
 │   ├── system-interactions.md               # Major workflows (boot, sensor fusion, OTA updates).
 │   ├── deployment-topology.md               # Flash layout, firmware update mechanisms (OTA, USB, JTAG).
 │   ├── 📁 c4-model/                         # 📌 C4 Model for Architecture Visualization
 │   │   ├── context.md                        # Level 1: System context diagram (how the system interacts with the world).
 │   │   ├── container.md                      # Level 2: Major system components (MCU firmware, cloud, mobile apps).
 │   │   ├── component.md                      # Level 3: Internal breakdown of each major component.
 │   │   ├── code.md                           # Level 4 (optional): Class/module-level details.
 │   │   ├── diagrams/                         # PlantUML, Structurizr, Mermaid diagrams.
 │
 ├── 📁 design-implementation/                # 📌 Detailed Embedded Software & Hardware Design
 │   ├── 📁 embedded-software/                 # 📌 Firmware Technical Design Documents (TDDs)
 │   │   ├── tdd-template.md                      # Standard TDD Template (see example below).
 │   │   ├── power-management.md                   # Sleep modes, wakeup sources, energy profiling.
 │   │   ├── interrupt-handling.md                 # ISR, DMA, low-latency event handling.
 │   │   ├── rtos-task-structure.md                # Task scheduling, inter-task communication.
 │   │   ├── uart-communication.md                 # UART buffering, DMA handling.
 │   │   ├── bootloader.md                         # Secure boot, firmware verification, rollback handling.
 │   │   ├── ci-cd-strategy.md                     # Automated builds, hardware-in-loop testing.
 │
 │   ├── 📁 api/                                # 📌 Device Communication APIs
 │   │   ├── overview.md                          # API structure and purpose (e.g., MQTT for telemetry).
 │   │   ├── mqtt-protocol.md                     # MQTT topic structure, QoS levels, payload format.
 │   │   ├── coap-rest-api.md                     # CoAP/REST API for device control.
 │   │   ├── ble-gatt-profile.md                  # BLE services, characteristics, UUID mapping.
 │   │   ├── protobuf-definitions/                # gRPC/Protocol Buffers for efficient IoT messaging.
 │   │   │   ├── telemetry.proto                  # Example telemetry message format.
 │   │   │   ├── ota-update.proto                 # OTA update request format.
 │   │   ├── data-models/                         # 📌 Data Schema & Storage Strategy
 │   │   │   ├── file-system-structure.md         # Flash filesystem layout (LittleFS, NOR Flash).
 │   │   │   ├── database-schema.md               # Cloud backend schema.
 │   │   │   ├── message-queues.md                # LoRaWAN, Zigbee, CAN bus messaging formats.
 │
 │   ├── 📁 security/                            # 📌 Security & Compliance
 │   │   ├── overview.md                          # Secure boot, key provisioning, trusted execution.
 │   │   ├── authentication.md                    # Device identity, X.509 certificates, TPM.
 │   │   ├── encryption.md                        # AES, ECC, TLS, and secure key storage.
 │   │   ├── firmware-integrity.md                # Anti-rollback, secure firmware signing.
 │   │   ├── threat-models.md                     # Threat modeling (STRIDE, LINDDUN).
 │   │   ├── compliance/                          # Certification (FCC, CE, ISO21434).
 │
 │   ├── 📁 rfc/                                # 📌 Requests for Comments (RFCs) for Design & Implementation
 │   │   ├── 0015-adopt-lora.md                    # Evaluating LoRaWAN for LPWAN use.
 │   │   ├── 0016-secure-bootloader.md             # Improving bootloader security.
 │
 ├── 📁 open-questions/                          # 📌 Unresolved Issues & Design Discussions
 │   ├── 2024-05-ota-security.md                   # How to improve OTA security.
 │   ├── 2024-06-sleep-mode-optimization.md        # Low power sleep mode refinements.
 │
 ├── 📁 reference/                               # 📌 Glossary, FAQs, & Common Resources
 │   ├── glossary.md                               # Embedded systems terminology.
 │   ├── faq.md                                    # Common questions about the system.
 │   ├── best-practices.md                         # Best practices for debugging, power optimization.
