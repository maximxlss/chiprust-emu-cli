# CLI for my Chip-8 emulator.
![Crates.io](https://img.shields.io/crates/v/chiprust-emu-cli?style=flat-square)

Simple Chip-8 emulator running in terminal. Not really fully working right now, backend has some bugs and the cli isn't finished.

## Usage
- To install the emulator with all the features using cargo, use `cargo install chiprust-emu-cli`.
- To disable sound and input, use `cargo install chiprust-emu-cli --no-default-features`
- To enable only sound or input, use `cargo install chiprust-emu-cli --no-default-features --features input/sound`

See also `chiprust-emu-cli --help`

## Requirements
- Base emulator needs an ANSI terminal bigger than 132x36 and std lib.
- Sound feature carries many dependencies and doesn't work on somewhat exotic platforms (Android). You can disable it with the method above.
- Input works only with X11 on linux.

## Working
- Basic emulation.
- Basic debugging.
- Basic input.

## Known bugs
- Weird emulation bugs in the backend
- Emulator hangs if you unlock cycle rate or set it too high

## TODO
- Add pause/step/resume keybinds
- Switch to more low-level audio lib to reduce dependency count
- Write docs for the backend
- Make keybinds customizable
