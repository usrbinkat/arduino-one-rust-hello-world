# About: .cargo/config.toml

This folder contains your [Cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html) for building and flashing the **Arduino Uno** (ATmega328P) using Rust.

## What is `.cargo/config.toml`?

The file `config.toml` in this directory instructs Cargo to:

1. **Compile for the AVR target** (`avr-atmega328p`).
2. **Automatically run `avrdude`** after each successful build, which flashes the compiled firmware onto your Arduino.

### Key Sections

- **`[build]`**: Sets the default build target to `avr-atmega328p`, ensuring all Cargo commands compile for the Arduino Uno’s microcontroller.
- **`[target.'cfg(target_arch = \"avr\")'] runner`**: Tells Cargo to execute `avrdude` right after building. This step effectively **automates** the upload process.

### Understanding the `runner` Command

Inside `config.toml`, you’ll see a line like this:

```toml
runner = "avrdude -c arduino -p atmega328p -P /dev/ttyACM0 -b 115200 -D -U flash:w:target/avr-atmega328p/debug/rust-arduino-morse.elf"
```

Let’s break down each part:

- **`avrdude`**: The utility that writes the compiled firmware to your microcontroller.
- **`-c arduino`**: Chooses the [programmer protocol](https://www.nongnu.org/avrdude/user-manual/avrdude_4.html) for Arduino’s bootloader.
- **`-p atmega328p`**: Identifies the microcontroller on the Arduino Uno.
- **`-P /dev/ttyACM0`**: Points to the device’s serial port (`COM3` on Windows, etc.).
- **`-b 115200`**: The baud rate required by the Arduino Uno bootloader.
- **`-D`**: Prevents `avrdude` from automatically erasing the flash—Arduino’s bootloader usually handles that.
- **`-U flash:w:...`**: Specifies the flash write operation, pointing to the `.elf` binary.

### Changing the Serial Port

If your board is connected to another port, modify the `-P` parameter:

```toml
runner = "avrdude -c arduino -p atmega328p -P COM3 -b 115200 -D -U flash:w:..."
```

On Linux, you might use `/dev/ttyUSB0` instead of `/dev/ttyACM0`.

### Release vs. Debug Builds

By default, the sample config references `debug` binaries:

```toml
-U flash:w:target/avr-atmega328p/debug/rust-arduino-morse.elf
```

For smaller, optimized binaries, switch to:

```toml
-U flash:w:target/avr-atmega328p/release/rust-arduino-morse.elf
```

Then run:

```bash
cargo build --release
cargo run --release
```

### Further Reading

- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)
- [arduino-hal GitHub Repository](https://github.com/Rahix/avr-hal/tree/main/arduino-hal)
- [AVR-GCC & Toolchain](http://www.nongnu.org/avr-libc/)
- [avrdude Documentation](http://www.nongnu.org/avrdude/user-manual/)
