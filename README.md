# Xenharmonic Piano Roll

**This is mostly an experiment.** \
It may never go anywhere, or it may become somehow useful one day.

This software is an experimental piano roll that has a continuous y axis.
Notes can be placed at any frequency, or at any interval from another note.

**It is barely usable in its current state.** \
You can still play around with it but many parts are missing.

## Goals

The goals of this project haven't been fully fleshed out yet, but for now, my only goal is for it to be fun to play with.

## Building

On Windows, the project can be built simply by running `cargo run` with Rust installed.

For other platforms, if this doesn't work, see the requirements of the [druid](https://github.com/xi-editor/druid) library.

## Usage

A note's pitch can be either:
- Absolute, in which case it's a frequency and can be moved freely
- Relative, in which case it's at a fixed interval to a root note

How to use:
- Place/move/resize notes with left click.
- Delete notes with right click.
- Add relative notes by double clicking a note.
- Navigate the board with the scrollbars, or with the mouse wheel (Ctrl/Shift/Alt to change the behavior of the wheel)
- Play the sheet with the spacebar

The layout of the piano roll can be altered in many different ways using the `Layout` button.
Layout markers can be added by right clicking the cursor bar.


## TODO

- Improve UI
- Selection
- Display errors on invalid input
- Control note volume

## License

This project is licensed under the Apache 2 [license](LICENSE).
