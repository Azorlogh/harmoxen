# Harmoxen

This software is an experimental piano roll that has continuous x and y axes.
Notes can be placed at any frequency, or at any interval from another note.


### Disclaimer

**This is a personal experiment.** \
It will probably never go anywhere, but I hope to learn from it and hopefully bring useful insights in the xenharmonic music space.

**It is barely usable in its current state.** \
You can still play around with it but many parts are missing.

![Screenshot](/assets/screenshot.png)


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

Right click on a layout marker (little flag on the cursor bar) to access its settings.
Layout markers can be added by right clicking the cursor bar.

You can make it can output MPE data through a MIDI port by going into the settings.

## TODO

- Work as a VST
- Improve UI
- Support more scale types & .scl import
- More UI feedback: display errors/warnings/infos
- Control note volume/other note attributes
- Support bending pitch/other note attributes

## License

This project is licensed under the Apache 2 [license](LICENSE).
