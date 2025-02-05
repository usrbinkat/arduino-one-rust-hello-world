# Developer Guide

## Overview

This guide provides a comprehensive overview of the project, including its architecture, components, and key functionalities and how to debug, develop, deploy, and maintain the project.

## Project Structure

The project is organized into several key directories and files:

- `src/main.rs`: The main entry point for the project.
- `src/lib.rs`: The library for the project.
- `src/bin/`: The binary for the project.
- `src/lib/`: The library for the project.
- `etc, etc, etc`: TODO: complete the layout architecture of the project.

## Getting Started with AVR Development

### Prerequisites

1. **Install AVR Development Tools**
   ```bash
   sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
   ```

2. **Install Cargo Generate**
   ```bash
   cargo install cargo-generate
   ```

3. **Install Ravedude**
   ```bash
   cargo install ravedude
   ```


### Project Setup

1. **Create a New Project**
   ```bash
   # Generate new project from the AVR HAL template
   cargo generate --git https://github.com/Rahix/avr-hal-template.git
   ```

   When prompted:
   - Enter project name: `rust-iot-light`
   - Select board: `Arduino Uno`

2. **Project Structure**
   After generation, your project will have the following structure:
   ```
   rust-iot-light/
   ├── Cargo.toml         # Project dependencies and configuration
   ├── rust-toolchain.toml # Rust toolchain configuration
   ├── .cargo/
   │   └── config.toml    # Cargo configuration for AVR
   └── src/
       └── main.rs        # Your application code
   ```

### Building and Flashing

1. **Connect Your Arduino**
   - Connect your Arduino Uno to your computer via USB
   - Note the USB device path (typically `/dev/ttyACM0` or `/dev/ttyUSB0` on Linux)

2. **Build the Project**
   ```bash
   cd rust-iot-light
   cargo build
   ```

3. **Flash to Arduino**
   ```bash
   cargo run
   ```

### Troubleshooting

1. **Permission Issues**
   If you encounter permission errors when accessing the USB device:
   ```bash
   sudo usermod -a -G dialout $USER
   # Log out and log back in for the changes to take effect
   ```

2. **Port Access**
   If ravedude can't find your board, verify the USB connection:
   ```bash
   ls /dev/tty*
   ```
   Look for devices like `ttyACM0` or `ttyUSB0`

### Notes

- This setup uses the [avr-hal](https://github.com/Rahix/avr-hal) project, which provides a Hardware Abstraction Layer for AVR microcontrollers
- The template automatically configures the correct Rust toolchain and target specifications
- Ravedude handles the flashing process, making it as simple as `cargo run`

### Next Steps

- Review the examples in the `avr-hal` repository for common usage patterns
- Explore the [arduino-hal documentation](https://docs.rs/arduino-hal) for available features
- Check the [embedded Rust book](https://docs.rust-embedded.org/book/) for embedded development concepts