#![no_std]
#![no_main]

use panic_halt as _;         // When panic occurs, halt the MCU
use arduino_hal::prelude::*; // Prelude for common traits

/// Entry point for the program.
#[arduino_hal::entry]
fn main() -> ! {
    // Acquire the peripheral singleton (all device registers).
    let dp = arduino_hal::Peripherals::take().unwrap();
    // Get access to the CPU's I/O pins.
    let pins = arduino_hal::pins!(dp);

    // Onboard LED at digital pin 13.
    // Configure it as an output pin.
    let mut led = pins.d13.into_output();

    // Define time units (milliseconds) for Morse code timing:
    // - 1 "dot" unit
    // - 3 "dot" units = dash
    // - spacing between dots/dashes, letters, and words
    let dot_duration = 200u16;
    let dash_duration = dot_duration * 3;
    let intra_char_gap = dot_duration;    // gap between dots/dashes
    let inter_char_gap = dot_duration * 3; // gap between letters
    let word_gap = dot_duration * 7;      // gap between words

    loop {
        // Blink "HELLO WORLD" in Morse code.
        // For reference:
        // H: dot dot dot dot
        // E: dot
        // L: dot dash dot dot
        // L: dot dash dot dot
        // O: dash dash dash
        // (space)
        // W: dot dash dash
        // O: dash dash dash
        // R: dot dash dot
        // L: dot dash dot dot
        // D: dash dot dot

        // --- H ---
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(inter_char_gap.into());

        // --- E ---
        dot(&mut led, dot_duration);
        delay_ms(inter_char_gap.into());

        // --- L ---
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(inter_char_gap.into());

        // --- L (again) ---
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(inter_char_gap.into());

        // --- O ---
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(word_gap.into());

        // --- W ---
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(inter_char_gap.into());

        // --- O ---
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(inter_char_gap.into());

        // --- R ---
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(inter_char_gap.into());

        // --- L ---
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(inter_char_gap.into());

        // --- D ---
        dash(&mut led, dash_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(intra_char_gap.into());
        dot(&mut led, dot_duration);
        delay_ms(word_gap.into());
    }
}

/// Produce a short "dot" blink.
fn dot(led: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output>, duration_ms: u16) {
    led.set_high();
    arduino_hal::delay_ms(duration_ms.into());
    led.set_low();
    // In Morse, a dot is followed by a short pause (handled in main loop).
}

/// Produce a long "dash" blink.
fn dash(led: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output>, duration_ms: u16) {
    led.set_high();
    arduino_hal::delay_ms(duration_ms.into());
    led.set_low();
    // In Morse, a dash is followed by a short pause (handled in main loop).
}

/// Simple convenience wrapper around arduino_hal's built-in delay.
fn delay_ms(ms: u32) {
    arduino_hal::delay_ms(ms);
}
