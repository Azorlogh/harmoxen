## Warning

This branch is work in progress, it is dedicated to migrating the software to the Iced library, as well as making it VST-compatible.

# Harmoxen

This software is an experimental piano roll that has a continuous y axis.
Notes can be placed at any frequency, or at any interval from another note.

**This is mostly an experiment.** \
It may never go anywhere, or it may become somehow useful one day.

**It is barely usable in its current state.** \
You can still play around with it but many parts are missing.

## Goals

- Be fun to play with
- Be easy to use in a DAW
- Encourage experimentation
- Allow as much creative freedom as possible

## Building

Running `cargo build` should be sufficient.
The project will only run in a `wgpu`-compatible environment. Check out [the wgpu repository](https://github.com/gfx-rs/wgpu) for more information.

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

You can make it can output MPE data through a MIDI port by going into the settings.

## TODO

- Improve UI
- Support more scale types & .scl import
- More feedback: display errors/warnings/infos
- Control note volume/other note attributes
- Support bending pitch/other note attributes

## License

This project is licensed under the Apache 2 [license](LICENSE).
