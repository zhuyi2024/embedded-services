# Overview

EC service is where the business logic glues the HAL + common EC functional traits + EC peripheral driver together.

# Building Blocks

## MCU Platform HAL

Hardware specific HAL leveraging Rust Async framework

- Must implement embedded-hal traits to allow a generic hardware agnostic interface
- Desire is for HALs to be open-sourced and upstreamed to Embassy main repo
- Plan to partner with MCU vendor to support more MCU in the future

For example, [embassy-imxrt](https://github.com/pop-project/embassy-imxrt)


```mermaid
        classDiagram
            class embassy-imxrt["embassy-imxrt I2C master HAL"]
            class embedded-hal["embedded-hal I2C master traits"]
            <<interface>> embedded-hal
            embedded-hal <|-- embassy-imxrt
            embedded-hal: +read()
            embedded-hal: +write()
            embassy-imxrt: +read()
            embassy-imxrt: +write()
```

## EC Subsystem Platform Abstractions

There are sets of generic Rust traits the define an EC functional subsystem like thermal, USB PD, fan, battery. This abstraction serves to abstract the underlying HW design away from the business logic.

For example, [embedded-sensor](https://github.com/pop-project/embedded-sensors)

```mermaid
    classDiagram
        embedded-sensor: +temperature()
        <<interface>> embedded-sensor
```

## Rust Based Drivers for EC Peripherals

There are MCU platform agnostic Rust drivers for specific HW parts connected to the EC like TMP108 temp sensor.

- Depending on embedded-hal interface so it is talking to a generic HW interface, not tying to any specific MCU platform
- Implements a EC function platform abstraction traits like `embedded-sensor`, `embedded-battery`, `embedded-fan`
- Plan to partner with vendor to open-source these drivers

For example, [tmp108](https://github.com/pop-project/tmp108)

```mermaid
    classDiagram
        embedded-sensor <|-- TMP108
        embedded-sensor: +temperature()
        <<interface>> embedded-sensor
        TMP108 --> embedded-hal
        TMP108: +temperature()
        class embassy-imxrt["embassy-imxrt I2C master HAL"]
        class embedded-hal["embedded-hal I2C master traits"]
        <<interface>> embedded-hal
        embedded-hal <|-- embassy-imxrt
        embedded-hal: +read()
        embedded-hal: +write()
        embassy-imxrt: +read()
        embassy-imxrt: +write()
```

# EC Services

EC service houses the business logic that glues the EC peripheral Rust driver + EC subsystem platform abstraction + MC platform HAL together/.

# Repo Organization
- embedded-services repo
  - embedded-services library crate
    - service traits
    - intrusive-list
    - transport/router
- power-button-service
  - library crate
- hid-service
  - library crate


## embedded-services

This houses common EC service utilities to build a service. It includes:
- instrusive-list that allows dynamic number of subscribers and publishers for a service
- transport (IPC) logic that allows EC services to talk to each other

## Individual services

Services will be separate crates in this repo. Each service crate will be implementation of the interfaces for a functional area.

The service itself should be hardware/platform agnostic and contains the application logic for EC functionality.

For example, temperature_service

```mermaid
    classDiagram
        temperature-service --> embedded-hal
        temperature-service --> embedded-sensor
        embedded-sensor <|-- TMP108
        embedded-sensor: +temperature()
        <<interface>> embedded-sensor
        TMP108 --> embedded-hal
        TMP108: +temperature()
        class embassy-imxrt["embassy-imxrt I2C master HAL"]
        class embedded-hal["embedded-hal I2C master traits"]
        <<interface>> embedded-hal
        embedded-hal <|-- embassy-imxrt
        embedded-hal: +read()
        embedded-hal: +write()
        embassy-imxrt: +read()
        embassy-imxrt: +write()
```

# EC Top-Level

At the top-level, a EC is an aggregate of service.

Sets of services be grouped into subsystem. For instance, thermal subsystem will consist of temperature-service + fan-service + battery-service + debug-service + host-comm-service. The service talks to each other through the transport (IPC) layer. An EC service will also be shared between different subsystems. For instance, debug-service will subcribe to debug messages from other services.

```
async fn (spawner: Spawner) {
    //initialize HW peripheral and system level managemetn
    spawn(services(periphal, configuration))
    ...
}
```

## Example: Simplified Layer of Subsystem + Services
![Simplified Layer View](docs/images/simplified_layers.png "Simplified Layered View of Services")

## Example: E2E of Keyboard over eSPI
![Keyboard to Host via eSPI Example](docs/images/keyboard_to_espi_example.png "Keyboard to Host via eSPI Example")
