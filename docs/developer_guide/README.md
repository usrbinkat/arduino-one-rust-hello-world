# BrightIdea Developer Guide

## Development Environment Setup (WSL2/Windows 11)

1. **Install Rust and Required Packages (In WSL)**
````bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.sh | sh

# Install AVR toolchain
sudo apt update
sudo apt install gcc-avr avr-libc avrdude pkg-config libudev-dev

# Install Ravedude
cargo install ravedude --version 0.1.7
````

2. **Install USB Support for Windows**
````powershell
# Run in Windows PowerShell as Administrator
winget install dorssel.usbipd-win
````

## USB/Serial Setup for WSL

1. **Connect Arduino to WSL**
````powershell
# In Windows PowerShell (Admin):
usbipd list  # Note the BUSID for Arduino
usbipd bind --busid <BUSID>
usbipd attach --busid <BUSID> --wsl
````

2. **Configure Serial Port Access (In WSL)**
````bash
# Add user to dialout group
sudo usermod -a -G dialout $USER
# Log out and back in for group changes to take effect

# Set permissions for serial port
sudo chmod a+rw /dev/ttyACM0
````

## Building and Testing

1. **Verify Setup**
````bash
# Check if Arduino is detected
ls -l /dev/ttyACM0

# Clean any previous builds
cargo clean
````

2. **Build and Run**
````bash
# Build the project
cargo build

# Upload to Arduino (when ready)
cargo run
````

## Project Structure

- `src/main.rs` - Main program entry point with LED blink example
- `.cargo/config.toml` - Cargo configuration for AVR
- `Cargo.toml` - Project dependencies and settings
- `rust-toolchain.toml` - Rust toolchain configuration

## Next Steps
- Implement IoT connectivity
- Add light control features
- Develop sensor integration
- Configure network communication

## Troubleshooting

If you encounter issues:
1. Verify Arduino is properly detected in WSL with `ls -l /dev/ttyACM0`
2. Ensure user is in the `dialout` group
3. Check USB connection and try different USB ports
4. Verify all required packages are installed
5. Run `cargo clean` before rebuilding

## Contributing
See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines on contributing to this project.
