# embedded-services-examples

## Introduction

These examples illustrate how to use the embassy-services for IPC, OEM customization, and feature extension.

## Adding Examples
Add uniquely named example to `src/bin` like `keyboard.rs`

## Build
`cd` to examples folder
`cargo build --bin <example_name>` for example, `cargo build --bin keyboard`

## Run
Assuming RT685 is powered and connected to Jlink debug probe and the latest probe-rs is installed via  
  `$ cargo install probe-rs-tools --git https://github.com/probe-rs/probe-rs --locked`  
`cd` to examples folder  
`cargo run --bin <example_name>` for example, `cargo run --bin keyboard`
