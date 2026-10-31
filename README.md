# Group Name

Rust VST Synth

## Group Members

| Name | NetID |
| --- | --- |
| Ross Kimme | rkimme2 |
| Alan | alanzz2 |
return *this; 
## Project Introduction
  The project will be a VST Synthesizer programmed in rust. Features planned to be included are Midi functionality, LFO controls and effects, Customizable wavetables where you draw 2+ shapes and the synthesizer interpolates between them. The final goal is to create a customizable synth with a working gui.
### Project Description
We are planing to develop a VST synthesizer plugin to be used within digital audio workstations. It will generate sound using oscillator waveforms, and have MIDI input support so notes can be played using a MIDI keyboard or other controller. Sound customization will come through waveform selection, envelopes, filters, and low-frequency oscillators. The main feature will be a custom wavetable editor, allowing users to draw multple waveforms and have the synth interpolate between them during playback.
### Goals and Objectives
Our main goals are to develop a working VST synthesizer plugin using rust, implementing MIDI input, basic oscillator waveforms such as sine, square, sawtooth, etc... implementing filters, LFOs, Attack Decay Sustain Release envelopes, a functioning GUI, and most importantly a customizable wavetable system. If time permits we would also like to implement other formats, such as CLAP.
### Motivation
Digital signal processing is an interest or Alan, who also does work with music production. In addition to the aforementioned, this project gives us an opportunity to explore audio engineering, and software development. 
## Technical Overview
The synth will be developed in rust. We plan to use the following crates and technologies:
- Nih-plug, which is a framework for developing audio plugins
- fundsp, whihc is a library whihc helps with oscillator generation
- egui, which is a GUI library which we will use to create the synthesizer's interface
- MIDI: we will use MIDI input and control to let users play into the synth directly
### Major Components
1. Audio engine and oscillators
2. midi Input
3. Envelopes and Filters
4. LFO and other modulation
5. Custom wave table system
6. Graphical interface
### Implementation Roadmap
We will first create a working audio plugin with basic sound generation which works with VST3, then we will implement Midi input. Next, we will expand the synth with additional waveforms and such. After the basic synth works, we will implement a basic Gui, LFO modulation, and custom wave tables. The last thing to be developed will be a polished GUI and additional CLAP functionality if time permits.
### Checkpoint Goals

| Checkpoint | Planned Deliverables |
| --- | --- |
| Week 1 | Midi input and sound output for sine wave, VST functionality, maybe envelopes |
| Week 2 | Functionality for the other basic waves, saw, square, envelopes and filters etc... Maybe customizable wavetables |
| Week 3 | LFOs and other controls/effects, basic gui |
| Week 4-5 | GUI, maybe CLAP, extra functionality with macros and effects and such |

## Possible Challenges

- Getting familiar with crates like nih_plug, fundsp, and egui, understanding their APIs, and figuring out how to use them together.
- Learning how to generate sound using oscillators, waveforms, frequencies, and envelopes.
- Implementing real-time audio processing
- Implementing interesting interpolation between custom waveforms.
- Making GUI for drawing custom waveform, then generating that sound.
- Managing the scope of the project
## References
- nih-plug - Rust audio plugin framework
- Fundsp - Rust audio DSP library
- egui - Immediate mode GUI library for Rust
- The Rust Programming Language
- VST 3 Developer Portal
